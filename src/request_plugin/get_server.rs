use std::process::Command;
use tracing::{error};

use serde_json::{to_writer_pretty};
use std::fs;
use std::env;
use std::path::{ PathBuf};
use std::io::{ BufWriter};
use colored::Colorize;

use chlaty_core::{request_plugin::get_server};


use crate::DEFAULT_STORAGE_DIRECTORY;
use crate::utils::{prompt_continue, get_installed_player};



pub fn new(plugin_id: &str, content_id: &str, episode_id: &str, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let storage_dir =  PathBuf::from(env::var("STORAGE_DIRECTORY").unwrap_or(DEFAULT_STORAGE_DIRECTORY.to_string()));
    let plugin_dir = storage_dir.join(plugin_id);
    let episode_dir = plugin_dir
        .join(content_id)
        .join(episode_id);

    println!("{}", format!("? Switch to different server if current server not working.").yellow());
    println!("{}", format!("> Fetching server with id {}...", &id).purple());
    let result = get_server::new(plugin_id, id);
    match result {
        Ok(result) => {
            match fs::create_dir_all(&episode_dir) {
                Err(e) => {error!("{}", e)},
                _ => {},
            }

            let manifest_path = episode_dir
            .join(format!("{}.json", &id));

            let file = match fs::OpenOptions::new()
                .write(true).create(true).truncate(true)
                .open(&manifest_path) {
                    Ok(file) => file,
                    Err(e) => {error!("{}", e); return Err(e.into())},
                };
            let writer = BufWriter::new(file);

            match to_writer_pretty(writer, &result) {
                Err(e) => {error!("{}", e)},
                _ => {},
            }

            let mut full_manifest_path:PathBuf = PathBuf::new();

            match fs::canonicalize(manifest_path) {
                Ok(full_path) => {
                    full_manifest_path = full_path;
                },
                Err(e) => error!("Error canonicalize path: {}", e),
            }

            if full_manifest_path.exists(){
                println!("{}", format!("> Launching chlaty-player... | {}", full_manifest_path.to_str().ok_or("Failed to get manifest path")?).purple());
                match get_installed_player::new() {
                    Ok(player_info) => {
                        Command::new(player_info.file)
                            .arg(format!("--manifest={}", full_manifest_path.to_str().ok_or("Failed to get manifest path")?))
                            .output()
                            .expect("Failed to execute command");
                    },
                    Err(e) => {
                        error!("{}", e);
                        error!("Try restarting chlaty-cli.");
                    },
                }
            }
        },
        Err(e) => error!("{}", e),
    }
        
    prompt_continue::new();

    return Ok(());
}