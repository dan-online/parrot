use crate::structs::info::SearchInfo;

pub fn parse_search(output: String) -> Vec<SearchInfo> {
    let lines = output.lines();
    let mut packages = Vec::new();
    let mut package = SearchInfo {
        repo: String::new(),
        name: String::new(),
        version: String::new(),
        description: String::new(),
        download_size: Some(String::new()),
        installed_size: Some(String::new()),
        votes: Some(String::new()),
        popularity: Some(String::new()),
        installed: false,
        out_of_date: false,
    };
    // loop lines
    for line in lines {
        let mut title = false;
        let split_space = line.split(" ").collect::<Vec<&str>>();
        let split_slash = split_space[0].split("/").collect::<Vec<&str>>();
        if split_slash.len() == 2 {
            title = true;
        }

        if title {
            // package = SearchInfo {
            //     repo: String::from(split_slash[0]),
            //     name: String::from(split_slash[1]),
            //     version: String::from(split_space[1]),
            //     download_size: Some(String::from(split_space[2].split_once("[").unwrap().1)),
            //     installed_size: Some(String::from(split_space[3].split_once("]").unwrap().0)),
            //     votes: None,
            //     popularity: None,
            //     installed: split_space.len() > 4 && split_space[4].contains("Installed"),
            //     description: String::new(),
            //     out_of_date: split_space[3].contains("Out Of Date"),
            // };
            package.repo = String::from(split_slash[0]);
            package.name = String::from(split_slash[1]);
            package.version = String::from(split_space[1]);

            if split_space.len() > 2 && split_space[2].contains("[") {
                if package.repo != "aur" {
                    package.download_size =
                        Some(String::from(split_space[2].split_once("[").unwrap().1));
                    package.installed_size =
                        Some(String::from(split_space[3].split_once("]").unwrap().0));
                } else {
                    package.votes = Some(String::from(split_space[2].split_once("[").unwrap().1));
                    package.popularity =
                        Some(String::from(split_space[3].split_once("]").unwrap().0));
                }
                package.installed = split_space.len() > 4 && split_space[4].contains("Installed");
                package.out_of_date = split_space[3].contains("Out Of Date");
            }
        } else {
            package.description = String::from(line.trim());
            // package.installed = check_installed(&package.name);
            packages.push(package);
            package = SearchInfo {
                repo: String::new(),
                name: String::new(),
                version: String::new(),
                description: String::new(),
                download_size: Some(String::new()),
                installed_size: Some(String::new()),
                votes: Some(String::new()),
                popularity: Some(String::new()),
                installed: false,
                out_of_date: false,
            };
        }
    }

    return packages;
}
