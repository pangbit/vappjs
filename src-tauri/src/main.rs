// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod codec;
mod format;
mod http;
mod regex;
mod x509;
mod random;

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
            http::ipinfo,
            x509::x509_parse,
            random::gen_passwords,
            random::gen_uuid_v4,
            random::gen_uuid_v7,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
