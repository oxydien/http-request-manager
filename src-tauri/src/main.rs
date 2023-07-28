#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod filesys;
mod request;
use crate::filesys::config::{create_config, get_config_path, read_config_file, write_config_file};
use crate::filesys::history::{clear_history_file, create_history_file, read_history_file};
use crate::request::http::make_http_request;

#[tauri::command]
fn get_config_values() -> String {
    if get_config_path().exists() {
        read_config_file()
    } else {
        create_config();
        read_config_file()
    }
}

#[tauri::command]
fn save_config(args: &str) -> Result<(), String> {
    Ok(write_config_file(args).unwrap())
}
#[tauri::command]
async fn send_request(
    method: &str,
    url: &str,
    data: Option<&str>,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<String, String> {
    make_http_request(method, url, data, headers)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn read_http_history() -> Result<String, String> {
    Ok(read_history_file().unwrap())
}

#[tauri::command]
fn clear_http_history() -> Result<(), String> {
    Ok(clear_history_file().unwrap())
}

fn main() {
    let _ = create_history_file();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_request,
            get_config_values,
            save_config,
            read_http_history,
            clear_http_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
