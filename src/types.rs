use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::error::DocError;

#[derive(Serialize, PartialEq, Clone, Copy)]
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
    pub props: Option<Properties>,
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

impl<'de> Deserialize<'de> for ChunkType {
    fn deserialize<D>(deserializer: D) -> Result<ChunkType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::Number(n) => {
                let chunk_type_value = n
                    .as_u64()
                    .ok_or_else(|| serde::de::Error::custom("Invalid chunk_type value"))?;

                match chunk_type_value {
                    24578 => Ok(ChunkType::Paragraph),
                    32771 => Ok(ChunkType::Text),
                    32773 => Ok(ChunkType::Image),
                    40966 => Ok(ChunkType::Link),
                    24584 => Ok(ChunkType::Ul),
                    24585 => Ok(ChunkType::Ol),
                    24586 => Ok(ChunkType::Li),
                    8191 => Ok(ChunkType::End),
                    _ => Err(serde::de::Error::custom("Unknown chunk_type value")),
                }
            }
            _ => Err(serde::de::Error::custom(
                "Invalid JSON value type for chunk_type",
            )),
        }
    }
}
