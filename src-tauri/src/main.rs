#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod parsers {
    pub mod get;
    pub mod icon;
    pub mod search;
}
mod structs {
    pub mod info;
}
mod commands {
    pub mod check_installed;
    pub mod get;
    pub mod installed;
    pub mod popular;
    pub mod run_instruction;
    pub mod search;
}
mod store {
    pub mod config;
    pub mod data;
}

use commands::check_installed::check_installed;
use commands::get::get;
use commands::installed::installed;
use commands::installed::installed_search;
use commands::popular::popular;
use commands::run_instruction::run_instruction;
use commands::search::search;

use reqwest::blocking::Client;
use store::config::ConfigStore;
use tauri::PathResolver;

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;
use structs::info::Log;

fn run(path: String, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let start = Instant::now();

    let output = Command::new(path).args(args).output()?;

    let duration = start.elapsed();

    println!(
        "[parrot] Command {} finished in {:?}, code {}",
        format!("paru {}", args.join(" ")),
        duration,
        output.status.code().unwrap_or(-1)
    );

    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(out)
    } else {
        Err(From::from(String::from_utf8_lossy(&output.stderr)))
    }
}

fn run_log(path: String, args: Vec<String>, sender: Sender<Log>) {
    let start = std::time::Instant::now();

    let mut cmd = Command::new(path);

    cmd.args(args.clone())
        .args(&["--noconfirm", "--sudoloop", "--sudo", "pkexec"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to execute child");

    let stdout = child.stdout.take().expect("failed to get stdout");
    let stderr = child.stderr.take().expect("failed to get stderr");

    let sender_clone = sender.clone();
    thread::spawn(move || {
        let mut stdout_reader = BufReader::new(stdout).lines();
        // let mut stderr_reader = BufReader::new(stderr).lines();

        while let Some(line) = stdout_reader.next() {
            sender_clone
                .send(Log {
                    line: line.unwrap(),
                    stdout: true,
                    done: false,
                })
                .unwrap();
        }
    });

    let sender_clone = sender.clone();
    thread::spawn(move || {
        let mut stderr_reader = BufReader::new(stderr).lines();

        while let Some(line) = stderr_reader.next() {
            sender_clone
                .send(Log {
                    line: line.unwrap(),
                    stdout: false,
                    done: false,
                })
                .unwrap();
        }
    });

    thread::spawn(move || {
        let status = child.wait().expect("failed to wait on child");
        sender
            .send(Log {
                line: format!("Command finished with {}", status),
                stdout: true,
                done: true,
            })
            .unwrap();

        let duration = start.elapsed();
        println!(
            "[parrot] Command {} finished in {:?}",
            format!("paru {}", args.join(" ")),
            duration
        );
    });
}

fn download_paru(resolver: PathResolver) {
    let config = ConfigStore::new(resolver.clone());

    let url = "https://api.github.com/repos/Morganamilo/paru/releases/latest";

    let app_user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = Client::builder()
        .user_agent(app_user_agent)
        .build()
        .unwrap();

    let res = client.get(url).send().unwrap();

    match res.json::<serde_json::Value>() {
        Ok(json) => {
            let remote_version = json["tag_name"].as_str().unwrap();
            let has_paru = config.get::<String>("paru").unwrap_or(None).is_some();

            if has_paru {
                match config.get::<String>("version") {
                    Ok(local_version) => match local_version {
                        Some(local_version) => {
                            if local_version == remote_version {
                                return;
                            }
                        }
                        None => {}
                    },
                    Err(_) => {}
                }
            }

            let assets = json["assets"].as_array().unwrap();

            let mut download_url = String::new();
            let target = std::env::consts::ARCH;

            for asset in assets {
                let name = asset["name"].as_str().unwrap();
                if name.contains(target) {
                    download_url = asset["browser_download_url"].as_str().unwrap().to_string();
                    break;
                }
            }

            println!("[parrot] Download URL: {}", download_url);

            let resp = reqwest::blocking::get(&download_url).unwrap();

            // uncompress using zstd
            let decoder_zst = zstd::Decoder::new(resp).unwrap();

            let path = resolver.app_data_dir().unwrap().join("paru");
            let bin_path = path.join("paru");

            tar::Archive::new(decoder_zst).unpack(&path).unwrap();

            println!("[parrot] Downloaded paru to {:?}", path);

            config.set("paru", bin_path.to_str().unwrap()).unwrap();
            config.set("version", remote_version).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search,
            check_installed,
            get,
            popular,
            run_instruction,
            installed,
            installed_search
        ])
        .setup(|app| {
            download_paru(app.path_resolver());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
