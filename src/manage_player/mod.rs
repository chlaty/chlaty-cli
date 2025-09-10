use inquire::{InquireError, Select};
use serde::{Deserialize, Serialize};
use tracing::{error};

use clearscreen;
use colored::Colorize;

use tabled::{Table, settings::Style, Tabled};
use semver::{ Version, VersionReq };

use crate::utils::{
    get_installed_player, get_player, install_player,
    prompt_continue
};



#[derive(Tabled, Debug, Deserialize, Serialize)]
pub struct PlayerInfoDisplay {
    pub version: String,
    pub file: String,
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {

    loop {
        clearscreen::clear().expect("failed to clear screen");
        
        let installed_player = get_installed_player::new()?;

        println!("{}", format!("> Installed Chlaty Player").purple());
        let mut player_info: Vec<PlayerInfoDisplay> = vec![];

        player_info.push(PlayerInfoDisplay {file: installed_player.file.clone(), version: installed_player.version.clone()});

        let mut table = Table::new(player_info);
        table.with(Style::rounded());
        println!("{}", format!("{}", table).cyan());

        let options: Vec<&str> = vec!["Update", "Reinstall", "Back"];
        let select: Result<&str, InquireError> = Select::new("Select an option: ", options).prompt();

        match select {
            Ok(choice) => {
                match choice {
                    "Update" => {
                        let req_ver = VersionReq::parse(&format!(">{}", installed_player.version.as_str())).unwrap();
                        let player_info = get_player::new("latest")?;

                        let latest_ver = Version::parse(player_info.version.as_str())?;
                        if req_ver.matches(&latest_ver) {
                            install_player::new("latest")?;
                        }else{
                            println!("{}", "? Already up to date.".yellow());
                        }
                    },
                    "Reinstall" => {
                        install_player::new(installed_player.version.as_str())?;
                    },
                    "Back" => {break;},
                    
                    _ => error!("There was an error, please try again."),
                }
            },
            Err(_) => println!("There was an error, please try again."),
        }
        prompt_continue::new();
    }
    return Ok(());
}
