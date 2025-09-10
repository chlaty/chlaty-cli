use serde_json::{from_reader};
use std::collections::HashMap;

use std::io::{BufReader};
use reqwest;



use crate::{ PLAYER_MANIFEST_URL };


pub struct PlayerInfo {
    pub version: String,
    pub manifest: String
}

pub fn new(version: &str) -> Result<PlayerInfo, Box<dyn std::error::Error>> {

    let client = reqwest::blocking::Client::new();
    let res = client.get(PLAYER_MANIFEST_URL).send()?;

    if res.status().is_success() {
        let manifest_reader = BufReader::new(res);
        let manifest_data: HashMap<String, String>  = from_reader(manifest_reader)?;
        
        let selected_version: String;

        if version == "latest" {
            let latest_version = manifest_data.get("latest-version").ok_or("Failed to get latest version")?.as_str().to_string();
            selected_version = latest_version;
        }else{
            selected_version = version.to_string();
        }
        
        
        let manifest_url = manifest_data.get(&selected_version).ok_or("Failed to get latest manifest url")?.as_str().to_string();
        
        return Ok(PlayerInfo { version: selected_version, manifest: manifest_url });
    }
    
    return Err("Failed to get manifest url".into());

}