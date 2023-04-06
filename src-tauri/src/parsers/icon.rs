use reqwest::blocking::Client;

pub fn _get_icon(url: &str) -> Option<String> {
    let app_user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let mut icon = None;
    let split = url.split("/").collect::<Vec<&str>>();

    let client = Client::builder()
        .user_agent(app_user_agent)
        .build()
        .unwrap();

    if split.len() == 5 {
        let owner = split[3];
        let repo = split[4];
        let url = format!("https://api.github.com/repos/{}/{}", owner, repo);
        let response = client.get(&url).send();
        match response {
            Ok(response) => {
                let json = response.json::<serde_json::Value>();
                match json {
                    Ok(json) => {
                        let icon_url = json["owner"]["avatar_url"].as_str();
                        match icon_url {
                            Some(icon_url) => {
                                icon = Some(String::from(icon_url));
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    return icon;
}
