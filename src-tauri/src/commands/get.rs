use tauri::{Manager, Window};

use crate::commands::check_installed::check_installed;
use crate::parsers::get::parse_get;
use crate::run;
use crate::store::config::ConfigStore;
use crate::store::data::DataStore;
use crate::structs::info::FullInfo;

#[tauri::command]
pub async fn get(window: Window, repo: String, name: String, id: String) {
    let store = DataStore::new(window.app_handle().path_resolver());
    let config = ConfigStore::new(window.app_handle().path_resolver());

    let path = config.get("paru").unwrap().unwrap_or("paru".to_string());
    let full = format!("{}/{}", repo, name);

    match store.get::<Vec<Option<FullInfo>>>(&full) {
        Ok(package) => match package {
            Some(package) => {
                window
                    .emit(format!("get-{}", id).as_str(), package)
                    .unwrap();
            }
            None => {}
        },
        Err(_) => {}
    }

    let local = check_installed(window.clone(), name.as_str());

    let mut results: Vec<Option<FullInfo>> = vec![None, None];
    let locations = ['Q', 'S'];

    for t in locations {
        let idx = if t == 'Q' { 0 } else { 1 };
        let mut option = String::from("-");
        option.push(t);

        option.push('i');
        option.push('i');

        let args = [
            option.as_str(),
            if local { name.as_str() } else { full.as_str() },
        ];
        let ran = run(path.clone(), &args);

        match ran {
            Ok(output) => {
                let package = parse_get(output, repo.clone());
                results[idx] = Some(package);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    }

    store.set(&full, results.clone()).unwrap();

    window
        .emit(format!("get-{}", id).as_str(), results)
        .unwrap();
}
