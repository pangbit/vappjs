use regex::Regex;

#[tauri::command]
pub fn regex_is_match(pattern: &str, text: &str) -> Result<bool, String> {
    match Regex::new(pattern) {
        Ok(rx) => Ok(rx.is_match(text)),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[tauri::command]
pub fn regex_captures(pattern: &str, text: &str) -> Result<Vec<String>, String> {
    let re = Regex::new(pattern).map_err(|e| format!("{e:?}"))?;

    let mut result = vec![];
    if let Some(caps) = re.captures(text) {
        for cap in caps.iter() {
            if let Some(cap) = cap {
                result.push(cap.as_str().to_owned());
            }
        }
    }

    Ok(result)
}
