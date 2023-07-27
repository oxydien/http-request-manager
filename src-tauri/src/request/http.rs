use crate::filesys::history::append_to_history;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct RequestResponse {
    request: RequestInfo,
    response: ResponseInfo,
    time: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestInfo {
    method: String,
    url: String,
    data: Option<String>,
    headers: Option<Vec<HeaderItem>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseInfo {
    result: String,
    status_code: u16,
    headers: Vec<HeaderItem>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HeaderItem {
    name: String,
    value: String,
}

pub async fn make_http_request(
    method: &str,
    url: &str,
    data: Option<&str>,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<String, String> {
    print!("Sending {} request", method);

    let client = reqwest::Client::new();
    let mut request_builder = match method {
        "GET" => client.get(url),
        "PUT" => client.put(url),
        "OPTIONS" => client.request(reqwest::Method::OPTIONS, url),
        "HEAD" => client.request(reqwest::Method::HEAD, url),
        "PATCH" => client.patch(url),
        "POST" => client.post(url),
        "DELETE" => client.delete(url),
        "TRACE" => client.request(reqwest::Method::TRACE, url),
        _ => panic!("Invalid HTTP method"),
    };

    let data_str = data.map(|d| d.to_owned());

    if let Some(data) = data {
        if let Ok(json_value) = serde_json::from_str::<&str>(data) {
            let body_string = json_value.to_string();
            request_builder = request_builder
                .header(
                    reqwest::header::CONTENT_TYPE,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .body(body_string);
        } else {
            request_builder = request_builder.body(data.to_owned());
        }
    }

    let mut headers_info = None;
    if let Some(headers) = headers {
        let mut header_map = HeaderMap::new();
        for (header_name, header_value) in &headers {
            header_map.insert(
                HeaderName::from_lowercase(header_name.to_lowercase().as_bytes()).unwrap(),
                HeaderValue::from_str(header_value).unwrap(),
            );
        }
        request_builder = request_builder.headers(header_map);
        headers_info = Some(
            headers
                .iter()
                .map(|(name, value)| HeaderItem {
                    name: name.to_string(),
                    value: value.to_string(),
                })
                .collect(),
        );
    }

    let start_time = std::time::Instant::now();
    let response = request_builder.send().await.map_err(|e| e.to_string())?;
    let end_time = std::time::Instant::now();
    let elapsed_time_ms = end_time.duration_since(start_time).as_millis();

    let status_code = response.status().as_u16();
    let response_headers = response.headers().clone();
    let response_text = response.text().await.map_err(|e| e.to_string())?;

    let request_info = RequestInfo {
        method: method.to_string(),
        url: url.to_string(),
        data: data_str,
        headers: headers_info,
    };

    let response_info = ResponseInfo {
        result: response_text,
        status_code,
        headers: response_headers
            .iter()
            .map(|(name, value)| HeaderItem {
                name: name.as_str().to_string(),
                value: value.to_str().unwrap().to_string(),
            })
            .collect(),
    };

    let request_response = RequestResponse {
        request: request_info,
        response: response_info,
        time: elapsed_time_ms,
    };

    let _ = append_to_history(request_response.clone());

    let json_result = serde_json::to_string(&request_response).map_err(|e| e.to_string())?;

    Ok(json_result)
}
