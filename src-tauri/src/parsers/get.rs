use crate::structs::info::{AurInfo, BaseInfo, FullInfo};

pub fn parse_get(output: String, repo: String) -> FullInfo {
    let lines = output.lines();
    let mut package: FullInfo;

    if repo == "aur" {
        package = FullInfo::Aur(AurInfo::new());
    } else {
        let mut base = BaseInfo::new();
        base.repo = repo;
        package = FullInfo::Base(base);
    }

    let mut key: String = String::new();

    for line in lines {
        let mut split = line.split(':').map(|s| s.trim()).collect::<Vec<&str>>();
        let value: String;

        if line.starts_with(' ') {
            // This is a continuation of the previous line
            value = split.join(":");
        } else {
            key = split.remove(0).to_string();
            value = split.join(":");
        }

        match key.clone().as_str() {
            "Name" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.name = value.to_string();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.name = value.to_string();
                }
            }
            "Version" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.version = value.to_string();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.version = value.to_string();
                }
            }
            "Description" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.description = format!("{}{}", aur.description, value.to_string());
                }
                if let FullInfo::Base(base) = &mut package {
                    base.description = format!("{}{}", base.description, value.to_string());
                }
            }
            "Architecture" => {
                if let FullInfo::Base(base) = &mut package {
                    base.architecture = value.to_string();
                }
            }
            "URL" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.url = value.to_string();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.url = value.to_string();
                }
            }
            "AUR URL" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.aur_url = value.to_string();
                }
            }
            "Licenses" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.licenses = value.to_string();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.licenses = value.to_string();
                }
            }
            "Groups" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.groups = value.to_string();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.groups = value.to_string();
                }
            }
            "Provides" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.provides = value.to_string();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.provides = value.to_string();
                }
            }
            "Depends On" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.depends_on = value
                        .split("  ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.depends_on = value
                        .split("  ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                }
            }

            "Optional Deps" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.optional_deps = value
                        .split("  ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                }
                if let FullInfo::Base(base) = &mut package {
                    base.optional_deps = value
                        .split("\n")
                        .map(|s| s.split(":").map(|c| c.to_string()).collect::<Vec<String>>())
                        .collect::<Vec<Vec<String>>>();
                }
            }
            "Required By" => {
                if let FullInfo::Base(base) = &mut package {
                    base.required_by = value
                        .split("  ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                }
            }
            "Conflicts With" => {
                if let FullInfo::Base(base) = &mut package {
                    base.conflicts_with = value
                        .split("  ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                }
            }
            "Download Size" => {
                if let FullInfo::Base(base) = &mut package {
                    base.download_size = value.to_string();
                }
            }

            "Installed Size" => {
                if let FullInfo::Base(base) = &mut package {
                    base.installed_size = value.to_string();
                }
            }

            "Packager" => {
                if let FullInfo::Base(base) = &mut package {
                    base.packager = value.to_string();
                }
            }

            "Maintainer" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.maintainer = value.to_string();
                }
            }

            "Votes" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.votes = value.to_string();
                }
            }

            "Popularity" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.popularity = value.to_string();
                }
            }

            "First Submitted" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.first_submitted = value.to_string();
                }
            }

            "Last Modified" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.last_modified = value.to_string();
                }
            }

            "Out Of Date" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.out_of_date = value.to_string();
                }
            }

            "ID" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.id = value.to_string();
                }
            }

            "Package Base ID" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.package_base_id = value.to_string();
                }
            }

            "Keywords" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.keywords = value
                        .split("  ")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                }
            }

            "Snapshot URL" => {
                if let FullInfo::Aur(aur) = &mut package {
                    aur.snapshot_url = value.to_string();
                }
            }

            "Build Date" => {
                if let FullInfo::Base(base) = &mut package {
                    base.build_date = value.to_string();
                }
            }

            "Install Date" => {
                if let FullInfo::Base(base) = &mut package {
                    base.install_date = value.to_string();
                }
            }

            _ => {
                // println!("Unrecognized key: {}", key);
            }
        };
    }

    return package;
}
