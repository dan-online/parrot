use std::sync::mpsc::channel;

use tauri::{Manager, Window};

use crate::{run_log, store::config::ConfigStore, structs::info::Log};

#[tauri::command]
pub async fn run_instruction(
    window: Window,
    instruction: String,
    optional_deps: Vec<String>,
    package: String,
) {
    let config = ConfigStore::new(window.app_handle().path_resolver());

    let path = config.get("paru").unwrap().unwrap_or("paru".to_string());
    let (tx, rx) = channel::<Log>();

    match instruction.as_str() {
        "install" => {
            let mut args = vec![
                "-S".to_string(),
                package.to_string(),
                "--needed".to_string(),
            ];

            if optional_deps.len() > 0 {
                args.push("--asdeps".to_string());
                args.extend(optional_deps)
            }

            run_log(path, args, tx);
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
