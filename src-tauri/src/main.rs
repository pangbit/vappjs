// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod codec;
mod format;
mod http;
mod regex;
mod x509;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            codec::base64_decode,
            codec::base64_encode,
            codec::url_decode,
            codec::url_encode,
            regex::regex_is_match,
            regex::regex_captures,
            format::json_format,
            format::xml_format,
            http::http_request,
            x509::x509_parse,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
