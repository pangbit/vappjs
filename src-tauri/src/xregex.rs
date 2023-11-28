use regex::Regex;

#[tauri::command]
pub fn regex_is_match(pattern: &str, text: &str) -> Result<bool, String> {
    match Regex::new(pattern) {
        Ok(rx) => Ok(rx.is_match(text)),
        Err(e) => Err(format!("{e:?}")),
    }
}
