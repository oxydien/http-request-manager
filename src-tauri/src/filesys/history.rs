use crate::filesys::get_app_path;
use crate::request::http::RequestResponse;
use serde::{Deserialize, Serialize};
use serde_json::Error as JSONError;
use std::fs;
use std::io::Error as IOError;

// Define an error type for your module
#[derive(Debug)]
pub enum HistoryError {
    IO(IOError),
    JSON(JSONError),
}

impl From<IOError> for HistoryError {
    fn from(error: IOError) -> Self {
        HistoryError::IO(error)
    }
}

impl From<JSONError> for HistoryError {
    fn from(error: JSONError) -> Self {
        HistoryError::JSON(error)
    }
}

#[derive(Serialize, Deserialize)]
struct HistoryData {
    requests: Vec<RequestResponse>,
}

pub fn create_history_file() -> Result<(), HistoryError> {
    let app_path = get_app_path();
    let file_path = app_path.join("history.json");

    if !file_path.exists() {
        let empty_history: Vec<RequestResponse> = Vec::new();
        let history_json = serde_json::to_string_pretty(&empty_history)?;
        fs::write(&file_path, history_json)?;
    }

    Ok(())
}

const MAX_HISTORY_ENTRIES: usize = 100;

pub fn append_to_history(data: RequestResponse) -> Result<(), HistoryError> {
    let app_path = get_app_path();
    let file_path = app_path.join("history.json");

    // Read existing history from the file
    let existing_history_str = fs::read_to_string(&file_path)?;
    let mut existing_history: Vec<RequestResponse> = serde_json::from_str(&existing_history_str)?;

    // Remove oldest entries if the history exceeds the maximum size
    if existing_history.len() >= MAX_HISTORY_ENTRIES {
        let num_entries_to_remove = existing_history.len() - MAX_HISTORY_ENTRIES + 1;
        existing_history.drain(0..num_entries_to_remove);
    }

    // Push the new data to the history
    existing_history.push(data);

    // Serialize and write back to the file
    let history_json = serde_json::to_string_pretty(&existing_history)?;
    fs::write(&file_path, history_json)?;

    Ok(())
}

pub fn read_history_file() -> Result<String, String> {
    let app_path = get_app_path();
    let file_path = app_path.join("history.json");

    // Read existing history from the file
    let existing_history_str = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let mut existing_history: Vec<RequestResponse> =
        serde_json::from_str(&existing_history_str).map_err(|e| e.to_string())?;

    // Reverse the history vector to have newer requests at the top
    existing_history.reverse();

    // Serialize the updated history back to a JSON string
    let updated_history_str =
        serde_json::to_string_pretty(&existing_history).map_err(|e| e.to_string())?;

    Ok(updated_history_str)
}

pub fn clear_history_file() -> Result<(), String> {
    let app_path = get_app_path();
    let file_path = app_path.join("history.json");

    // Create an empty history
    let empty_history: Vec<RequestResponse> = Vec::new();

    // Serialize the empty history to a JSON string
    let empty_history_str =
        serde_json::to_string_pretty(&empty_history).map_err(|e| e.to_string())?;

    // Write the empty history to the file, effectively clearing its content
    fs::write(&file_path, empty_history_str).map_err(|e| e.to_string())?;

    Ok(())
}
