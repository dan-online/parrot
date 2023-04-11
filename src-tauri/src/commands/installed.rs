use tauri::{Manager, Window};

use crate::parsers::get::parse_get;
use crate::parsers::search::parse_search;
use crate::run;
use crate::store::config::ConfigStore;
use crate::structs::info::FullInfo;

#[tauri::command]
pub async fn installed(window: Window) {
    let config = ConfigStore::new(window.app_handle().path_resolver());
    let path = config.get("paru").unwrap().unwrap_or("paru".to_string());

    let mut results: Vec<FullInfo> = vec![];

    let ran = run(path.clone(), &["-Qii"]);

    match ran {
        Ok(output) => {
            let packages = output.split("\n\n").collect::<Vec<&str>>();
            for package_str in packages {
                let package = parse_get(
                    window.clone(),
                    package_str.to_string(),
                    "local".to_string(),
                    true,
                );
                results.push(package);
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };

    window.emit("installed", results).unwrap();
}

#[tauri::command]
pub async fn installed_search(window: Window, query: String, id: String) {
    let config = ConfigStore::new(window.app_handle().path_resolver());
    let path = config.get("paru").unwrap().unwrap_or("paru".to_string());

    let ran = run(path.clone(), &["-Qs", query.as_str()]);

    match ran {
        Ok(output) => {
            let packages = parse_search(output);
            println!("[parrot] Found {} packages", packages.len());
            window
                .emit(&format!("installed_search_{}", id), packages)
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    };
}
