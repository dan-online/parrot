use std::time::Instant;

use tauri::{Manager, Window};

use crate::{
    parsers::search::parse_search,
    run,
    store::{config::ConfigStore, data::DataStore},
    structs::info::SearchInfo,
};

#[tauri::command]
pub async fn search(window: Window, query: String, local: bool) {
    let store = DataStore::new(window.app_handle().path_resolver());
    let config = ConfigStore::new(window.app_handle().path_resolver());

    let path = config.get("paru").unwrap().unwrap_or("paru".to_string());
    let key = format!("search-{}", query);

    match store.get::<Vec<SearchInfo>>(&key) {
        Ok(packages) => match packages {
            Some(packages) => {
                window.emit("search", packages).unwrap();
                return;
            }
            None => {}
        },
        Err(_) => {}
    }

    let mut option = String::from("-");

    if local {
        option.push('Q');
    } else {
        option.push('S');
    }

    option.push('s');

    let args = [option.as_str(), &query];
    let ran = run(path, &args);

    let start = Instant::now();

    let packages = match ran {
        Ok(output) => parse_search(output),
        Err(e) => {
            println!("Error: {}", e);
            vec![]
        }
    };

    let duration = start.elapsed();

    println!("Parsing finished in {:?}", duration);

    store.set(&key, packages.clone()).unwrap();

    window.emit("search", packages).unwrap();
}
