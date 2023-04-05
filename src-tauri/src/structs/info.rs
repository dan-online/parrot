use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BaseInfo {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub architecture: String,
    pub url: String,
    pub licenses: String,
    pub groups: String,
    pub provides: String,
    pub depends_on: Vec<String>,
    pub optional_deps: Vec<Vec<String>>, // [[name, description], ...]
    pub required_by: Vec<String>,
    pub conflicts_with: Vec<String>,
    pub replaces: String,
    pub download_size: String,
    pub installed_size: String,
    pub packager: String,
    pub build_date: String,
    pub install_date: String,
    pub md5_sum: String,
    pub sha256_sum: String,
    pub signatures: String,
    pub installed: bool,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct AurInfo {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub aur_url: String,
    pub groups: String,
    pub licenses: String,
    pub provides: String,
    pub depends_on: Vec<String>,
    pub optional_deps: Vec<String>, // [[name], ...]
    pub conflicts_with: String,
    pub maintainer: String,
    pub votes: String,
    pub popularity: String,
    pub first_submitted: String,
    pub last_modified: String,
    pub out_of_date: String,
    pub id: String,
    pub package_base_id: String,
    pub keywords: Vec<String>,
    pub snapshot_url: String,
}

impl AurInfo {
    pub fn new() -> Self {
        Self {
            repo: "aur".to_string(),
            name: String::new(),
            version: String::new(),
            description: String::new(),
            url: String::new(),
            aur_url: String::new(),
            groups: String::new(),
            licenses: String::new(),
            provides: String::new(),
            depends_on: Vec::new(),
            optional_deps: Vec::new(),
            conflicts_with: String::new(),
            maintainer: String::new(),
            votes: String::new(),
            popularity: String::new(),
            first_submitted: String::new(),
            last_modified: String::new(),
            out_of_date: String::new(),
            id: String::new(),
            package_base_id: String::new(),
            keywords: Vec::new(),
            snapshot_url: String::new(),
        }
    }
}

impl BaseInfo {
    pub fn new() -> Self {
        Self {
            repo: String::new(),
            name: String::new(),
            version: String::new(),
            description: String::new(),
            architecture: String::new(),
            url: String::new(),
            licenses: String::new(),
            groups: String::new(),
            provides: String::new(),
            depends_on: Vec::new(),
            optional_deps: Vec::new(),
            required_by: Vec::new(),
            conflicts_with: Vec::new(),
            replaces: String::new(),
            download_size: String::new(),
            installed_size: String::new(),
            packager: String::new(),
            build_date: String::new(),
            install_date: String::new(),
            md5_sum: String::new(),
            sha256_sum: String::new(),
            signatures: String::new(),
            installed: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum FullInfo {
    Base(BaseInfo),
    Aur(AurInfo),
}

/*
community/discord 0.0.26-1 [0B 222.60MiB] [Installed]
    All-in-one voice and text chat for gamers
aur/discord_arch_electron 0.0.25-1 [+161 ~11.69] [Out-of-date: 2023-03-30]
    Discord (popular voice + video app) using the system provided electron for increased security and performance
*/
#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct SearchInfo {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub download_size: Option<String>,
    pub installed_size: Option<String>,
    pub votes: Option<String>,
    pub popularity: Option<String>,
    pub installed: bool,
    pub description: String,
    pub out_of_date: bool,
}

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct PopularInfo {
    pub repo: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub votes: i32,
    pub popularity: String,
}

#[derive(Serialize, Clone, TS)]
#[ts(export)]
pub struct Log {
    pub line: String,
    pub stdout: bool,
    pub done: bool,
}
