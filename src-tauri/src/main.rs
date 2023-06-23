// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use http::Method;
use reqwest::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use reqwest::Client;
use serde::Serialize;
use serde_json::{Result as Result2, Value};
use std::env::{self};
use std::fs;

#[derive(Debug, Serialize)]
struct RequestResponse {
    result: String,
    status_code: u16,
    headers: Vec<(String, String)>,
}

#[tauri::command]
fn get_config_values() -> String {
    if check_config_file_exists("config.json") {
        read_config_file()
    } else {
        create_config();
        read_config_file()
    }
}

#[tauri::command]
fn save_config(args: &str) -> bool {
    let current_dir = std::env::current_dir().expect("[save-config]: Failed to get current directory");
    let config_path = current_dir.join("config.json");
    let result = std::fs::write(config_path, args);
    result.is_ok()
}
#[tauri::command]
async fn send_request(
    method: &str,
    url: &str,
    data: Option<&str>,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<RequestResponse, String> {
    make_http_request(method, url, data, headers)
        .await
        .map_err(|err| err.to_string())
}

async fn make_http_request(
    method: &str,
    url: &str,
    data: Option<&str>,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<RequestResponse, reqwest::Error> {
    print!("Sending {} request", method);

    let client = Client::new();
    let mut request_builder = match method {
        "GET" => client.get(url),
        "PUT" => client.put(url),
        "PATCH" => client.patch(url),
        "POST" => client.post(url),
        "DELETE" => client.delete(url),
        "TRACE" => client.request(Method::TRACE, url),
        _ => panic!("Invalid HTTP method"),
    };

    if let Some(data) = data {
        if let Ok(json_value) = str_to_json(data) {
            let body_string = json_value.to_string();
            request_builder = request_builder
                .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .body(body_string);
        } else {
            request_builder = request_builder.body(data.to_owned());
        }
    }

    if let Some(headers) = headers {
        let mut header_map = reqwest::header::HeaderMap::new();
        for (header_name, header_value) in headers {
            header_map.insert(
                HeaderName::from_lowercase(header_name.as_bytes()).unwrap(),
                HeaderValue::from_str(header_value).unwrap(),
            );
        }
        request_builder = request_builder.headers(header_map);
    }

    let response = request_builder.send().await?;
    let status_code = response.status().as_u16();
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| (name.to_string(), value.to_str().unwrap().to_string()))
        .collect();
    let response_text = response.text().await?;

    Ok(RequestResponse {
        result: response_text,
        status_code,
        headers,
    })
}

fn str_to_json(string: &str) -> Result2<Value> {
    let v: Value = serde_json::from_str(string)?;
    Ok(v)
}

fn create_config() {
    let default_config = r#"
  {
    "version": "1.0.0",
    "defaults": {
      "headers": [
        {
          "name": "user-agent",
          "value":
            "Mozilla/5.0 (Android 13; Mobile; rv:109.0) Gecko/113.0 Firefox/113.0"
        }
      ],
      "body": "",
      "page": "SETTINGS"
    },
    "pages": ["GET", "POST", "PATCH", "PUT", "DEL", "TRACE"],
    "css":""
  }"#;
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let config_path = current_dir.join("config.json");
    fs::write(config_path, default_config).expect("Failed to write config file");
}

fn check_config_file_exists(file_name: &str) -> bool {
    let current_dir = env::current_dir().expect("[check-file]: Failed to get current directory");
    let config_path = current_dir.join(file_name);
    fs::metadata(config_path).is_ok()
}

fn read_config_file() -> String {
  let current_dir = env::current_dir().expect("[read-config]: Failed to get current directory");
  let config_path = current_dir.join("config.json");
  let file_contents = fs::read_to_string(config_path).expect("[read-config]: Failed to read config file");
  file_contents
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![send_request,get_config_values,save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
