mod error;
mod types;

use types::Chunk;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DocxDocument {
    chunks: Vec<Chunk>,
}

#[wasm_bindgen]
impl DocxDocument {
    pub fn new() -> DocxDocument {
        DocxDocument { chunks: vec![] }
    }

    pub fn from_chunks(self, chunks: JsValue) -> JsValue {
            
        JsValue::from_str("test")
    }
}
