extern crate reqwest;

use reqwest::Client;
use scraper::{Html, Selector};
use tauri::{Manager, Window};

use crate::{store::data::DataStore, structs::info::PopularInfo};

#[tauri::command]
pub async fn popular(window: Window) {
    let store = DataStore::new(window.app_handle().path_resolver());
    match store.get::<Vec<PopularInfo>>(&"aur") {
        Ok(packages) => match packages {
            Some(packages) => {
                window.emit("popular", packages).unwrap();
                return;
            }
            None => {}
        },
        Err(_) => {}
    }

    let client = Client::new();

    let res = client.get("https://aur.archlinux.org/packages?O=0&SeB=nd&K=&outdated=off&SB=l&SO=d&PP=250&submit=Go").send().await.unwrap();
    let body = res.text().await.unwrap();

    let document = Html::parse_document(&body);
    let selector = Selector::parse(".results tr").unwrap();
    let elements = document.select(&selector).skip(1);

    let mut packages = Vec::new();

    for element in elements {
        let name = element
            .select(&Selector::parse("td:nth-child(1) a").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>();

        if name.len() == 0 {
            continue;
        }

        let name = name[0].trim();

        let version = element
            .select(&Selector::parse("td:nth-child(2)").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>();

        if version.len() == 0 {
            continue;
        }

        let version = version[0].trim();

        let votes = element
            .select(&Selector::parse("td:nth-child(3)").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>();

        if votes.len() == 0 {
            continue;
        }

        let votes: i32 = votes.join("").parse().unwrap();

        let popularity = element
            .select(&Selector::parse("td:nth-child(4)").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>();

        if popularity.len() == 0 {
            continue;
        }

        let popularity = popularity[0].trim();

        let description = element
            .select(&Selector::parse("td:nth-child(5)").unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<_>>();

        if description.len() == 0 {
            continue;
        }

        let description = description[0].trim();

        if votes > 5 {
            let info = PopularInfo {
                repo: "aur".to_string(),
                name: name.to_string(),
                version: version.to_string(),
                votes,
                popularity: popularity.to_string(),
                description: description.to_string(),
            };

            packages.push(info);
        }
    }

    store.set(&"aur", packages.clone()).unwrap();

    window.emit("popular", packages).unwrap();
}
