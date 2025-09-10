use serde_json::{from_str};
use serde::{Deserialize, Serialize};

use std::fs;


use std::str::from_utf8;
use std::env;
use std::path::{ PathBuf };
use sled;

use crate::{ DEFAULT_BIN_DIRECTORY };



#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerInfo {
    pub file: String,
    pub version: String
    
}


pub fn new() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bin_dir = PathBuf::from(env::var("CHLATY_BIN_DIRECTORY").unwrap_or(DEFAULT_BIN_DIRECTORY.to_string()));
    fs::create_dir_all(&bin_dir)?;

    let bin_manifest = bin_dir.join("manifest");

    let tree = sled::open(&bin_manifest)?;
    let value = tree.get("chlaty-player".as_bytes())?.ok_or("Failed to get chalty-player")?;
    let player_info: PlayerInfo = from_str(from_utf8(&value)?)?;
    
    let player_path = PathBuf::from(&player_info.file);

    if !player_path.exists() {
        return Err("Chlaty-player not found".into());
    }
    
    return Ok(player_path);

}