use std::str::FromStr;

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
    Break = 11 | 0x4000,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub chunk_type: ChunkType,
    pub text: Option<String>,
    pub props: Option<Properties>,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub struct Px {
    val: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Properties {
    pub url: Option<String>,
    pub color: Option<String>,
    pub background: Option<String>,
    pub font_size: Option<Px>,
    pub font_family: Option<String>,
    pub bold: Option<bool>,
    pub strike: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub align: Option<String>,
    pub indent: Option<Px>,
    pub line_height: Option<String>,
    pub width: Option<Px>,
    pub height: Option<Px>,
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

impl Default for Properties {
    fn default() -> Self {
        Properties {
            font_size: Some(Px::new(16)),
            url: None,
            color: None,
            background: None,
            font_family: None,
            bold: None,
            strike: None,
            italic: None,
            underline: None,
            align: None,
            indent: None,
            line_height: None,
            width: None,
            height: None,
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
                    16395 => Ok(ChunkType::Break),
                    _ => Err(serde::de::Error::custom("Unknown chunk_type value")),
                }
            }
            _ => Err(serde::de::Error::custom(
                "Invalid JSON value type for chunk_type",
            )),
        }
    }
}

impl Px {
    pub fn new(px: i32) -> Px {
        Px { val: px }
    }

    pub fn get_val(self) -> i32 {
        self.val
    }
}

impl FromStr for Px {
    type Err = DocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s[0..s.len() - 2]
            .parse::<i32>()
            .map_err(|_| DocError::new(&format!("unable to parse value: {}", s)))?;
        Ok(Px::new(v))
    }
}

impl<'de> Deserialize<'de> for Px {
    fn deserialize<D>(deserializer: D) -> Result<Px, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value {
            Value::String(val) => {
                let px = val.parse::<Px>().map_err(|_| {
                    serde::de::Error::custom(&format!("unable to parse Px value {}", val))
                })?;
                Ok(px)
            }
            _ => Err(serde::de::Error::custom(
                "Invalid JSON value type for Px struct",
            )),
        }
    }
}
