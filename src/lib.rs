mod error;
mod types;

use error::DocError;
use types::{Chunk, ChunkType, NumberingData, NumberingType, Properties};
use wasm_bindgen;
use wasm_bindgen::prelude::*;

use docx_rs::{
    AbstractNumbering, AlignmentType, Docx, Hyperlink, HyperlinkType, IndentLevel, Level, LevelJc,
    LevelText, NumberFormat, Numbering, NumberingId, Paragraph, ParagraphChild, ParagraphProperty,
    Pic, Run, RunFonts, RunProperty, Start, SpecialIndentType,
};

use gloo_utils::format::JsValueSerdeExt;

const BULLETS: [&str; 3] = ["\u{2022}", "\u{25E6}", "\u{25AA}"];

#[wasm_bindgen]
#[derive(Default)]
pub struct DocxDocument {
    chunks: Vec<Chunk>,
    stack: Vec<ChunkType>,
    it: usize,
    it_start: bool,
    numberings: Vec<NumberingData>,
}

fn save_docx(docx: Docx) {
    let path = std::path::Path::new("./temp/output/test.docx");
    let file = std::fs::File::create(&path).unwrap();
    docx.build().pack(file).unwrap();
}

#[wasm_bindgen]
impl DocxDocument {
    pub fn new() -> DocxDocument {
        Default::default()
    }

    pub fn from_js_chunks(&mut self, raw: JsValue) -> Vec<u8> {
        let chunks = raw.into_serde().unwrap();
        self.from_chunks(chunks)
    }

    fn from_chunks(&mut self, chunks: Vec<Chunk>) -> Vec<u8> {
        self.chunks = chunks;
        let docx = self.build().unwrap();

        save_docx(docx);

        vec![]
    }

    fn build(&mut self) -> Result<Docx, DocError> {
        let mut doc: Docx = docx_rs::Docx::new();

        while self.next().is_some() {
            let chunk = self.curr().unwrap();

            match chunk.chunk_type {
                ChunkType::Paragraph => {
                    doc = doc.add_paragraph(self.parse_block(&chunk)?);
                }
                ChunkType::Ol | ChunkType::Ul => {
                    let list =
                        self.parse_numbering(0, NumberingType::from_chunk_type(chunk.chunk_type)?)?;

                    for p in list.iter() {
                        doc = doc.add_paragraph(p.to_owned());
                    }
                }
                // ChunkType::End => {
                //     self.stack_pop()?;
                //     continue;
                // }
                _ => continue,
            }
        }

        if !self.stack.is_empty() {
            return Err(DocError::new("some block statements are not closed"));
        }

        doc = self.build_numbering(doc);

        Ok(doc)
    }

    fn parse_block(&mut self, block_chunk: &Chunk) -> Result<Paragraph, DocError> {
        let mut para = Paragraph::new();

        para.property = self.parse_block_props(&block_chunk.props)?;
        para.children = self.parse_block_content(block_chunk)?;

        Ok(para)
    }

    fn parse_numbering(
        &mut self,
        level: usize,
        num_type: NumberingType,
    ) -> Result<Vec<Paragraph>, DocError> {
        let mut buf: Vec<Paragraph> = vec![];

        self.stack.push(self.curr().unwrap().chunk_type);

        let num_id = self.add_numbering(num_type);

        while self.next().is_some() {
            let c = self.curr().unwrap();

            match c.chunk_type {
                ChunkType::Ol => {
                    buf.append(&mut self.parse_numbering(level + 1, NumberingType::Decimal)?);
                }
                ChunkType::Ul => {
                    buf.append(&mut self.parse_numbering(level + 1, NumberingType::Bullet)?);
                }
                ChunkType::Li => {
                    let mut para = self.parse_block(&c)?;
                    para = para.numbering(NumberingId::new(num_id), IndentLevel::new(level));
                    buf.push(para);
                }
                ChunkType::End => {
                    self.stack_pop()?;
                    return Ok(buf);
                }
                _ => {
                    // FIXME remove this return
                    return Err(DocError::new(&format!(
                        "unexpected chunk type: {}",
                        c.chunk_type.to_string()
                    )));
                }
            }
        }

        Err(DocError::new("unexpected end of statement"))
    }

    fn parse_block_content(
        &mut self,
        block_chunk: &Chunk,
    ) -> Result<Vec<ParagraphChild>, DocError> {
        self.stack.push(block_chunk.chunk_type);

        let mut children: Vec<ParagraphChild> = vec![];

        while self.next().is_some() {
            let c = self.curr().unwrap();

            match c.chunk_type {
                ChunkType::Text => {
                    let run = self.parse_text(&c);
                    let child = ParagraphChild::Run(Box::new(run));
                    children.push(child);
                }
                ChunkType::Image => {
                    let pic = self.parse_pic(&c)?;
                    let run = Run::new().add_image(pic);
                    let child = ParagraphChild::Run(Box::new(run));
                    children.push(child);
                }
                ChunkType::Link => {
                    let mut hp =
                        Hyperlink::new(c.props.url.to_owned().unwrap(), HyperlinkType::External);
                    hp.children = self.parse_block_content(&c)?;

                    let child = ParagraphChild::Hyperlink(hp);
                    children.push(child);
                }
                ChunkType::End => {
                    self.stack_pop()?;
                    return Ok(children);
                }
                _ => {
                    // FIXME remove this return
                    return Err(DocError::new(&format!(
                        "unexpected chunk type: {}",
                        c.chunk_type.to_string()
                    )));
                }
            }
        }

        Err(DocError::new("unexpected end of statement"))
    }

    fn parse_pic(&self, chunk: &Chunk) -> Result<Pic, DocError> {
        let buf = self.parse_pic_source(chunk)?;

        let pic = Pic::new(&buf);
        // TODO parse image
        Ok(pic)
    }

    fn parse_pic_source(&self, _chunk: &Chunk) -> Result<Vec<u8>, DocError> {
        // TODO parse image source
        Ok(vec![])
    }

    fn parse_text(&self, chunk: &Chunk) -> Run {
        let mut run = Run::new().add_text(chunk.text.to_owned().unwrap());
        run.run_property = self.parse_run_props(&chunk.props);
        run
    }

    fn parse_block_props(&self, props: &Properties) -> Result<ParagraphProperty, DocError> {
        let mut para_props = ParagraphProperty::new();

        if let Some(align) = &props.align {
            let res = <AlignmentType as std::str::FromStr>::from_str(&align);
            match res {
                Ok(v) => para_props = para_props.align(v),
                Err(_) => return Err(DocError::new(&format!("unknown alignment type: {}", align))),
            };
        }
        if let Some(_indent) = &props.indent {
            // TODO parse indent
        }
        if let Some(_lh) = &props.line_height {
            // TODO parse line height
        }

        Ok(para_props)
    }

    fn parse_run_props(&self, props: &Properties) -> RunProperty {
        let mut run_props = RunProperty::new();
        if let Some(bold) = props.bold {
            if bold {
                run_props = run_props.bold();
            }
        }
        if let Some(italic) = props.italic {
            if italic {
                run_props = run_props.italic();
            }
        }
        if let Some(underline) = props.underline {
            if underline {
                run_props = run_props.underline("single");
            }
        }
        if let Some(color) = &props.color {
            run_props = run_props.color(color);
        }
        if let Some(sz) = &props.font_size {
            let v = sz[0..sz.len() - 2].parse::<usize>().unwrap();
            run_props = run_props.size(v);
        }
        if let Some(fam) = &props.font_family {
            run_props = run_props.fonts(RunFonts::new().ascii(fam));
        }
        if let Some(highlight) = &props.background {
            // FIXME need to support of add RunProperty.Shading in docx-rs. 'Highlight' is another thing
            run_props = run_props.highlight(highlight);
        }

        run_props
    }

    fn next(&mut self) -> Option<Chunk> {
        if self.it_start {
            self.it += 1;
        } else {
            self.it_start = true;
        }
        self.curr()
    }

    fn curr(&self) -> Option<Chunk> {
        if self.it >= self.chunks.len() {
            return None;
        }
        let chunk = &self.chunks[self.it];
        Some(chunk.clone())
    }

    fn stack_pop(&mut self) -> Result<(), DocError> {
        if self.stack.is_empty() {
            return Err(DocError::new("unexpected 'end' chunk"));
        }
        self.stack.pop();
        Ok(())
    }

    fn add_numbering(&mut self, t: NumberingType) -> usize {
        let id = self.numberings.len() + 1;
        self.numberings.push(NumberingData::new(id, t));
        id
    }

    fn build_numbering(&self, mut docx: Docx) -> Docx {
        for num in &self.numberings {
            let mut n = AbstractNumbering::new(num.get_id());
            for i in 0..9 {
                n = n.add_level(numbering_level(i, num.get_type()))
            }

            docx = docx
                .add_abstract_numbering(n)
                .add_numbering(Numbering::new(num.get_id(), num.get_id()));
        }

        docx
    }
}

fn numbering_level(l: usize, t: NumberingType) -> Level {
    Level::new(
        l,
        Start::new(1),
        NumberFormat::new(t.to_string()),
        LevelText::new(get_numbering_text(l, t)),
        LevelJc::new("left"),
    )
    .indent(
        Some(((l + 2) * 360) as i32),
        Some(SpecialIndentType::Hanging(320)),
        // None,
        None,
        None,
    )
}

fn get_numbering_text(l: usize, t: NumberingType) -> String {
    match t {
        NumberingType::Bullet => BULLETS[l % BULLETS.len()].to_owned(),
        NumberingType::Decimal => format!("%{}", l + 1).to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        types::{Chunk, ChunkType, Properties},
        DocxDocument,
    };

    fn text(text: String, props: Properties) -> Chunk {
        Chunk {
            chunk_type: ChunkType::Text,
            text: Some(text.to_owned()),
            props: props,
        }
    }
    fn para(props: Properties) -> Chunk {
        Chunk {
            chunk_type: ChunkType::Paragraph,
            text: None,
            props: props,
        }
    }
    fn ol(props: Properties) -> Chunk {
        Chunk {
            chunk_type: ChunkType::Ol,
            text: None,
            props: props,
        }
    }
    fn ul(props: Properties) -> Chunk {
        Chunk {
            chunk_type: ChunkType::Ul,
            text: None,
            props: props,
        }
    }
    fn li(props: Properties) -> Chunk {
        Chunk {
            chunk_type: ChunkType::Li,
            text: None,
            props: props,
        }
    }
    fn end() -> Chunk {
        Chunk {
            chunk_type: ChunkType::End,
            text: None,
            props: Default::default(),
        }
    }
    fn hyperlink(url: String) -> Chunk {
        Chunk {
            chunk_type: ChunkType::Link,
            text: None,
            props: Properties {
                url: Some(url),
                ..Default::default()
            },
        }
    }

    #[test]
    fn test_para() {
        let chunks = vec![
            para(Properties {
                align: Some("end".to_owned()),
                ..Default::default()
            }),
            text(
                "Hello".to_owned(),
                Properties {
                    bold: Some(true),
                    ..Default::default()
                },
            ),
            text(
                "Rust".to_owned(),
                Properties {
                    italic: Some(true),
                    underline: Some(true),
                    font_size: Some("32px".to_owned()),
                    ..Default::default()
                },
            ),
            text(
                "!!!".to_owned(),
                Properties {
                    background: Some("#123".to_owned()),
                    ..Default::default()
                },
            ),
            end(),
            para(Properties {
                align: Some("center".to_owned()),
                ..Default::default()
            }),
            hyperlink("https://webix.com".to_owned()),
            text(
                "Visit webix".to_owned(),
                Properties {
                    underline: Some(true),
                    color: Some("#0066ff".to_owned()),
                    background: Some("#ff00ff".to_owned()),
                    ..Default::default()
                },
            ),
            end(),
            end(),
        ];

        let mut d = DocxDocument::new();

        d.from_chunks(chunks);
    }

    #[test]
    fn test_numbering() {
        let chunks = vec![
            para(Properties {
                align: Some("end".to_owned()),
                ..Default::default()
            }),
            text(
                "Hello".to_owned(),
                Properties {
                    bold: Some(true),
                    ..Default::default()
                },
            ),
            text(
                "Rust".to_owned(),
                Properties {
                    italic: Some(true),
                    underline: Some(true),
                    font_size: Some("32px".to_owned()),
                    ..Default::default()
                },
            ),
            text(
                "!!!".to_owned(),
                Properties {
                    background: Some("#123".to_owned()),
                    ..Default::default()
                },
            ),
            end(),
            ol(Properties::default()),
            /**/ li(Properties::default()),
            /**//**/ text("Kanban".to_owned(), Properties::default()),
            /**/ end(),
            /**/ li(Properties::default()),
            /**//**/ text("To Do List".to_owned(), Properties::default()),
            /**/ end(),
            /**/ ul(Properties::default()),
            /**//**/ li(Properties::default()),
            /**//**//**/ text("Label".to_owned(), Properties::default()),
            /**//**/ end(),
            /**//**/ li(Properties::default()),
            /**//**//**/ text("Due date".to_owned(), Properties::default()),
            /**//**/ end(),
            /**//**/ ul(Properties::default()),
            /**//**//**/ li(Properties::default()),
            /**//**//**//**/ text("Time zone".to_owned(), Properties::default()),
            /**//**//**/ end(),
            /**//**//**/ li(Properties::default()),
            /**//**//**//**/ text("Time".to_owned(), Properties::default()),
            /**//**//**/ end(),
            /**//**/ end(),
            /**//**/ li(Properties::default()),
            /**//**//**/ text("Checked".to_owned(), Properties::default()),
            /**//**/ end(),
            /**/ end(),
            /**/ li(Properties::default()),
            /**//**/ text("Gantt".to_owned(), Properties::default()),
            /**/ end(),
            end(),
        ];

        let mut d = DocxDocument::new();

        d.from_chunks(chunks);
    }
}
