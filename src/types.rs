use serde::{Deserialize, Serialize};

use crate::error::DocError;

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
    // TODO Newline
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
    pub width: Option<String>,
    pub height: Option<String>,
}

#[derive(Clone, Copy)]
pub enum NumberingType {
    Bullet,
    Decimal,
}

pub struct NumberingData {
    id: usize,
    num_type: NumberingType,
}

impl ChunkType {
    pub fn to_string(self) -> String {
        match self {
            ChunkType::Paragraph => "paragraph".to_string(),
            ChunkType::Text => "text".to_string(),
            ChunkType::Image => "image".to_string(),
            ChunkType::Link => "link".to_string(),
            ChunkType::Ul => "ul".to_string(),
            ChunkType::Ol => "ol".to_string(),
            ChunkType::Li => "li".to_string(),
            ChunkType::End => "end".to_string(),
        }
    }
}

impl NumberingType {
    pub fn to_string(self) -> String {
        match self {
            NumberingType::Bullet => "bullet".to_string(),
            NumberingType::Decimal => "decimal".to_string(),
        }
    }

    pub fn from_chunk_type(value: ChunkType) -> Result<NumberingType, DocError> {
        match value {
            ChunkType::Ul => Ok(NumberingType::Bullet),
            ChunkType::Ol => Ok(NumberingType::Decimal),
            _ => return Err(DocError::new("unknown numbering type")),
        }
        // TODO handle "none" type
    }
}

impl NumberingData {
    pub fn new(id: usize, t: NumberingType) -> NumberingData {
        NumberingData {
            id: id,
            num_type: t,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_type(&self) -> NumberingType {
        self.num_type
    }
}
