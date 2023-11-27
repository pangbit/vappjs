// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use xml::{EmitterConfig, ParserConfig};

#[tauri::command]
fn base64_decode(input: &str) -> String {
    match general_purpose::STANDARD.decode(input) {
        Ok(bs) => String::from_utf8_lossy(&bs).to_string(),
        Err(e) => format!("{e:?}"),
    }
}

#[tauri::command]
fn base64_encode(input: &str) -> String {
    general_purpose::STANDARD.encode(input)
}

#[tauri::command]
fn url_decode(input: &str) -> String {
    let bs = urlencoding::decode_binary(input.as_bytes());
    String::from_utf8_lossy(&bs).to_string()
}

#[tauri::command]
fn url_encode(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

#[tauri::command]
fn regex_is_match(pattern: &str, text: &str) -> Result<bool, String> {
    match Regex::new(pattern) {
        Ok(rx) => Ok(rx.is_match(text)),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[tauri::command]
fn json_format(input: &str) -> Result<String, String> {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(v) => match serde_json::to_string_pretty(&v) {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("{e:?}")),
        },
        Err(e) => Err(format!("{e:?}")),
    }
}

#[tauri::command]
fn xml_format(input: &str) -> Result<String, String> {
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

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            base64_decode,
            base64_encode,
            url_decode,
            url_encode,
            regex_is_match,
            json_format,
            xml_format,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
