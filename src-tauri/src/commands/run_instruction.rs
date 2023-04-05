use std::sync::mpsc::channel;

use tauri::{Manager, Window};

use crate::{run_log, store::config::ConfigStore, structs::info::Log};

#[tauri::command]
pub async fn run_instruction(window: Window, instruction: String, package: String) {
    let config = ConfigStore::new(window.app_handle().path_resolver());

    let path = config.get("paru").unwrap().unwrap_or("paru".to_string());
    let (tx, rx) = channel::<Log>();

    match instruction.as_str() {
        "install" => {
            run_log(path, vec!["-S".to_string(), package.to_string()], tx);
        }
        "remove" => {
            run_log(path, vec!["-R".to_string(), package.to_string()], tx);
        }
        "upgrade" => {
            run_log(path, vec!["-Syu".to_string(), package.to_string()], tx);
        }
        _ => {}
    }

    loop {
        match rx.recv() {
            Ok(msg) => {
                window.emit("log", &msg).unwrap();
                if msg.done {
                    break;
                }
            }
            Err(_) => break,
        }
    }

    return;
}
