use xml::{EmitterConfig, ParserConfig};

#[tauri::command]
pub fn json_format(input: &str) -> Result<String, String> {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(v) => match serde_json::to_string_pretty(&v) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("{e:?}")),
        },
        Err(e) => Err(format!("{e:?}")),
    }
}

#[tauri::command]
pub fn xml_format(input: &str) -> Result<String, String> {
    let mut output = vec![];

    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .ignore_comments(false)
        .create_reader(input.as_bytes());

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .normalize_empty_elements(false)
        .create_writer(&mut output);

    for event in reader {
        if let Some(event) = event.map_err(|e| format!("{e:?}"))?.as_writer_event() {
            writer.write(event).map_err(|e| format!("{e:?}"))?
        }
    }

    Ok(String::from_utf8_lossy(&output).to_string())
}

