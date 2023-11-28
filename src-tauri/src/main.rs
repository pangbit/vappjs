// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

mod xcodec;
mod xformat;
mod xhttp;
mod xregex;

#[tauri::command]
fn rlog(m: HashMap<String, String>) -> String {
    format!("{m:?}") + "----->"
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            xcodec::base64_decode,
            xcodec::base64_encode,
            xcodec::url_decode,
            xcodec::url_encode,
            xregex::regex_is_match,
            xformat::json_format,
            xformat::xml_format,
            xhttp::http_request,
            rlog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
