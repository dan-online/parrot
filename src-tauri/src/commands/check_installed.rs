use tauri::{Manager, Window};

use crate::{run, store::config::ConfigStore};

#[tauri::command]
pub fn check_installed(window: Window, name: &str) -> bool {
    let config = ConfigStore::new(window.app_handle().path_resolver());
    let path = config.get("paru").unwrap().unwrap_or("paru".to_string());

    let output = run(path, &["-Q", name]);
    match output {
        Ok(_) => true,
        Err(_) => false,
    }
}
