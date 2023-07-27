use std::path::PathBuf;
pub mod config;
pub mod history;

pub fn get_app_path() -> PathBuf {
    if cfg!(windows) {
        match std::env::var("APPDATA") {
            Ok(appdata) => {
                let mut path = PathBuf::from(appdata);
                path.push("oxy-req-mng");
                path
            }
            Err(_) => panic!("Failed to get the %APPDATA% environment variable."),
        }
    } else {
        let mut path = PathBuf::from("/etc");
        path.push("oxy-req-mng");
        path
    }
}
