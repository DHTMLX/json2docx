mod error;
mod types;

use error::DocError;
use types::{Chunk, ChunkType, Properties};
use wasm_bindgen;
use wasm_bindgen::prelude::*;

use docx_rs::{
    AlignmentType, Docx, Hyperlink, HyperlinkType, Paragraph, ParagraphChild, ParagraphProperty,
    Pic, Run, RunFonts, RunProperty,
};

use gloo_utils::format::JsValueSerdeExt;

#[wasm_bindgen]
#[derive(Default)]
pub struct DocxDocument {
    chunks: Vec<Chunk>,
    stack: Vec<ChunkType>,
    it: usize,
    it_start: bool,
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

            if chunk.chunk_type.is_paragraph() {
                let para = self.parse_block(&chunk)?;

                if chunk.chunk_type.is_list() {
                    // TODO parse list
                }

                doc = doc.add_paragraph(para);
            };
        }

        Ok(doc)
    }

    fn parse_block(&mut self, block_chunk: &Chunk) -> Result<Paragraph, DocError> {
        let mut para = Paragraph::new();

        para.property = self.parse_block_props(&block_chunk.props)?;
        para.children = self.parse_block_content(block_chunk)?;

        Ok(para)
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
                    if self.stack.is_empty() {
                        return Err(DocError::new("unexpected block end"));
                    }
                    self.stack.pop();
                }
                _ => (),
            }
        }

        Ok(children)
    }

    fn parse_pic(&self, chunk: &Chunk) -> Result<Pic, DocError> {
        let buf = self.parse_pic_source(chunk)?;

        let pic = Pic::new(&buf);
        // TODO parse image
        Ok(pic)
    }

    fn parse_pic_source(&self, chunk: &Chunk) -> Result<Vec<u8>, DocError> {
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
        if let Some(_lh) = &props.line_height {
            // TODO parse line height
        }
        if let Some(_indent) = &props.indent {
            // TODO parse indent
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
}

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
    fn end() -> Chunk {
        Chunk {
            chunk_type: ChunkType::Paragraph,
            text: None,
            props: Properties::new(),
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
        ];

        let mut d = DocxDocument::new();

        d.from_chunks(chunks);
    }
}
