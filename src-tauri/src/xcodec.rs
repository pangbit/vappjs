use base64::{engine::general_purpose, Engine as _};

#[tauri::command]
pub fn base64_decode(input: &str) -> String {
    match general_purpose::STANDARD.decode(input) {
        Ok(bs) => String::from_utf8_lossy(&bs).to_string(),
        Err(e) => format!("{e:?}"),
    }
}

#[tauri::command]
pub fn base64_encode(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
}

#[tauri::command]
pub fn url_decode(input: &str) -> String {
    let bs = urlencoding::decode_binary(input.as_bytes());
    String::from_utf8_lossy(&bs).to_string()
}

#[tauri::command]
pub fn url_encode(input: &str) -> String {
    urlencoding::encode(input).to_string()
}
