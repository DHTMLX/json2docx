use regex::Regex;

use crate::error::DocError;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn parse_str_size(v: &String, remove_last_chars: usize) -> Result<i32, DocError> {
    match v[0..v.len() - remove_last_chars].parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(DocError::new(&format!("unbale to parse string: {}", v))),
    }
}

pub fn is_url(v: &String) -> bool {
    let url_pattern = Regex::new(r#"^(https?|ftp)://[^\s/$.?#].[^\s]*$"#).unwrap();
    url_pattern.is_match(v)
}

// pub fn download_file(url: &String) -> Result<Vec<u8>, DocError> {
//     fn f(url: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//         let response = reqwest::blocking::get(url)?;

//         if response.status().is_success() {
//             return Ok(response.bytes()?.to_vec());
//         }

//         let doc_err = DocError::new(&format!(
//             "unbale to load image. Status code: {}",
//             response.status().as_str()
//         ));
//         Err(Box::new(doc_err))
//     }

//     match f(url) {
//         Ok(bytes) => Ok(bytes),
//         Err(e) => Err(DocError::new(&e.to_string())),
//     }
// }

pub fn px_to_emu(px: i32) -> i32 {
    let dpi = 96;
    px * (914400 / dpi)
}

pub fn px_to_docx_points(px: i32) -> i32 {
    let pt = px_to_pt(px);
    pt * 2
}

fn px_to_pt(px: i32) -> i32 {
    (px as f32 * 0.75) as i32
}
