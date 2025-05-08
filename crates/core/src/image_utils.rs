use std::io::Cursor;
use image::ImageReader;
use regex::Regex;


pub fn parse_image_b64_from_image_url_openai(image_url: &str) -> Option<(String, String, String)> {
    let re = Regex::new(r"data:(image/(png|jpeg|jpg|webp|gif));base64,([A-Za-z0-9+/=]+)").unwrap();
    re.captures(image_url).and_then(|captures| {
        let image_type = captures.get(1)?.as_str().to_string();
        let encoding = "base64".to_string();
        let value = captures.get(3)?.as_str().to_string();
        Some((image_type, encoding, value))
    })
}

pub fn image_reader_from_b64string(image_b64: &str) -> Result<ImageReader<Cursor<Vec<u8>>>, String> {
    #[allow(deprecated)]
    let image_bytes = base64::decode(image_b64).map_err(|_| "base64 decode failed".to_string())?;
    let cursor = Cursor::new(image_bytes);
    ImageReader::new(cursor).with_guessed_format().map_err(|e| e.to_string())
}