use docx_rs::{Level, LevelJc, LevelText, NumberFormat, SpecialIndentType, Start};

use crate::{error::DocError, types::ChunkType, utils};

const BULLETS: [&str; 3] = ["\u{2022}", "\u{25E6}", "\u{25AA}"];

#[derive(Clone, Copy)]
pub enum NumberingType {
    Bullet,
    Decimal,
}

pub struct NumberingData {
    id: usize,
    num_type: NumberingType,
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

pub fn numbering_level(l: usize, t: NumberingType) -> Level {
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
    .size(utils::px_to_docx_points(16) as usize) // 12 pt
}

pub fn get_numbering_text(l: usize, t: NumberingType) -> String {
    match t {
        NumberingType::Bullet => BULLETS[l % BULLETS.len()].to_owned(),
        NumberingType::Decimal => format!("%{}.", l + 1).to_owned(),
    }
}
