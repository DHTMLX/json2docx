use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum ChunkType {
    Paragraph = 2 | 0x4000 | 0x2000,
    Text = 3 | 0x8000,
    Image = 5 | 0x8000,
    Link = 6 | 0x2000 | 0x8000,
    Ul = 8 | 0x2000 | 0x4000,
    Ol = 9 | 0x2000 | 0x4000,
    Li = 10 | 0x2000 | 0x4000,
    End = 0x1fff,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub chunk_type: ChunkType,
    pub text: Option<String>,
    pub props: Properties,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Properties {
    pub url: Option<String>,
    pub color: Option<String>,
    pub background: Option<String>,
    pub font_size: Option<String>,
    pub font_family: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub align: Option<String>,
    pub indent: Option<String>,
    pub line_height: Option<String>,
}

impl Chunk {
    pub fn new(t: ChunkType) -> Chunk {
        Chunk {
            chunk_type: t,
            props: Properties::new(),
            text: None,
        }
    }

    // pub fn set_text(&mut self, text: String) {
    //     if self.chunk_type == ChunkType::Text {
    //         self.text = Some(text);
    //     }
    // }

    // pub fn set_url(&mut self, url: String) {
    //     if self.chunk_type == ChunkType::Image || self.chunk_type == ChunkType::Link {
    //         self.props.url = Some(url);
    //     }
    // }

    // pub fn set_props(&mut self, props: Properties) {
    //     match self.chunk_type {
    //         ChunkType::Paragraph
    //         | ChunkType::Li
    //         | ChunkType::Ol
    //         | ChunkType::Ul
    //         | ChunkType::Link => {
    //             self.props.indent = props.indent;
    //             self.props.align = props.align;
    //             self.props.line_height = props.line_height;
    //         }
    //         ChunkType::Text => {
    //             self.props.color = props.color;
    //             self.props.background = props.background;
    //             self.props.font_size = props.font_size;
    //             self.props.font_family = props.font_family;
    //             self.props.bold = props.bold;
    //             self.props.italic = props.italic;
    //             self.props.underline = props.underline;
    //         }
    //         _ => (),
    //     }
    // }
}

impl Properties {
    pub fn new() -> Properties {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.url == None
            && self.color == None
            && self.background == None
            && self.font_size == None
            && self.font_family == None
            && self.bold == None
            && self.italic == None
            && self.underline == None
            && self.align == None
            && self.indent == None
            && self.line_height == None
    }
}

impl ChunkType {
    pub fn is_paragraph(self) -> bool {
        match self {
            ChunkType::Paragraph | ChunkType::Ol | ChunkType::Ul => true,
            _ => false,
        }
    }
    pub fn is_list(self) -> bool {
        match self {
            ChunkType::Ol | ChunkType::Ul => true,
            _ => false,
        }
    }
}
