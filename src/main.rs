use inquire::{InquireError, Select};
use tracing::{info, error};
use tracing_subscriber;
use figlet_rs::FIGfont;
use clearscreen;
use colored::Colorize;
use dotenv::dotenv;

mod utils;
mod display;
mod request_plugin;
mod manage_plugin;
mod manage_player;

pub const DEFAULT_BIN_DIRECTORY: &str = "bin";
pub const DEFAULT_PLUGIN_DIRECTORY: &str = "plugins";
pub const DEFAULT_STORAGE_DIRECTORY: &str = "storage";

pub const PLAYER_MANIFEST_URL: &str = "https://raw.githubusercontent.com/chlaty/chlaty-player/refs/heads/main/manifest.json";


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    clearscreen::clear().expect("failed to clear screen");
    
    println!("{}", format!("> Checking for chlaty-player...").purple());
    match utils::get_installed_player::new() {
        Ok(player_info) => {
            println!("{}", format!("> Chlaty-player found! | {}", player_info.file).green())
        },
        Err(_) => {
            println!("{}", format!("? Chlaty-player not found!").yellow());
            println!("{}", format!("> Installing chlaty-player...").purple());
            utils::install_player::new("latest")?;
        },
    };

    loop {
        clearscreen::clear().expect("failed to clear screen");
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("Chlaty-CLI");
        assert!(figure.is_some());
        println!("{}", figure.unwrap().to_string().cyan());
        println!("{}", format!("v{}\n", env!("CARGO_PKG_VERSION")).purple());

        let options: Vec<&str> = vec!["Search", "Plugin", "Chlaty Player", "Exit"];
        let select: Result<&str, InquireError> = Select::new("Select an option: ", options).prompt();

        match select {
            Ok(choice) => {
                match choice {
                    "Search" => request_plugin::search::new()?,
                    "Plugin" => manage_plugin::main()?,
                    "Chlaty Player" => manage_player::main()?,
                    "Exit" => {info!("Exiting..."); break;},
                    
                    _ => error!("There was an error, please try again."),
                }
            },
            Err(_) => println!("There was an error, please try again."),
        }
    }
    return Ok(());
}
