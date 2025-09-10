use serde_json::{from_reader, Value, json, to_string};
use std::collections::HashMap;
use std::fs;
use std::env::consts;
use std::io::{BufReader};
use reqwest;
use std::env;
use std::path::{ PathBuf};
use sled;
use indicatif::{ProgressBar, ProgressStyle};
use colored::Colorize;

use crate::utils::{download, get_extension};
use crate::{ PLAYER_MANIFEST_URL, DEFAULT_BIN_DIRECTORY};



pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "> Checking available version...".purple());

    let client = reqwest::blocking::Client::new();
    let res = client.get(PLAYER_MANIFEST_URL).send()?;

    if res.status().is_success() {
        let manifest_reader = BufReader::new(res);
        let manifest_data: HashMap<String, String>  = from_reader(manifest_reader)?;
        
        let latest_version = manifest_data.get("latest-version").ok_or("Failed to get latest version")?.as_str().to_string();
        let manifest_url = manifest_data.get(&latest_version).ok_or("Failed to get latest manifest url")?.as_str().to_string();
        
        let client = reqwest::blocking::Client::new();
        let res = client.get(manifest_url).send()?;

        if res.status().is_success() {
            let manifest_reader = BufReader::new(res);
            let release_manifest_data: Value = from_reader(manifest_reader)?;
            let data = release_manifest_data.get(consts::OS).and_then(|a| a.get(consts::ARCH))
                .ok_or("Unable to find supported OS and Arch inside manifest")?;

            let file_url = data.get("file")
                .ok_or("Unable to find file url inside manifest")?
                .as_str().ok_or("Unable to convert file url to str")?;


            let bin_dir = PathBuf::from(env::var("CHLATY_BIN_DIRECTORY").unwrap_or(DEFAULT_BIN_DIRECTORY.to_string()));
            fs::create_dir_all(&bin_dir)?;

            let player_path = bin_dir.join(format!("chlaty-player{}", get_extension::new()?));

            println!("{}", format!("> Downloading... | {}", file_url).purple());

            let pb = ProgressBar::new(0);
            pb.set_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
            );
            
            download::new(file_url, player_path.to_str().ok_or("Unable to convert path to str")?,
                |current,total| {
                    pb.set_length(total as u64);
                    pb.set_position(current as u64); 
                }
            )?;

            let bin_manifest = bin_dir.join("manifest");

            {
                let tree = sled::open(&bin_manifest)?;

                tree.remove("chlaty-player".as_bytes())?;
                tree.flush()?;
            }

            let tree = sled::open(&bin_manifest)?;
            tree.insert("chlaty-player".as_bytes(), to_string(&json!({
                "file": player_path,
                "version": latest_version
            }))?.as_bytes())?;

            tree.flush()?;
        }

    }
    
    return Ok(());

}