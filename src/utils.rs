use regex::Regex;

use crate::error::DocError;

pub fn parse_str_size(v: &String, remove_last_chars: usize) -> Result<u32, DocError> {
    match v[0..v.len() - remove_last_chars].parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(DocError::new(&format!("unbale to parse string: {}", v))),
    }
}

pub fn is_url(v: &String) -> bool {
    let url_pattern = Regex::new(r#"^(https?|ftp)://[^\s/$.?#].[^\s]*$"#).unwrap();
    url_pattern.is_match(v)
}

pub fn download_file(url: &String) -> Result<Vec<u8>, DocError> {
    fn f(url: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(url)?;

        if response.status().is_success() {
            return Ok(response.bytes()?.to_vec());
        }

        let doc_err = DocError::new(&format!(
            "unbale to load image. Status code: {}",
            response.status().as_str()
        ));
        Err(Box::new(doc_err))
    }

    match f(url) {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(DocError::new(&e.to_string())),
    }
}
