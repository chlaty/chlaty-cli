use inquire::{InquireError, Select};
use tracing::{info, error};
use tracing_subscriber;
use figlet_rs::FIGfont;
use clearscreen;
use colored::Colorize;

mod utils;
mod display;
mod request_plugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    loop {
        clearscreen::clear().expect("failed to clear screen");
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("Chlaty-CLI");
        assert!(figure.is_some());
        println!("{}", figure.unwrap().to_string().cyan());
        let options: Vec<&str> = vec![ "Search", "Bookmark", "Manage Plugin", "Exit"];
        let select: Result<&str, InquireError> = Select::new("Select an option: ", options).prompt();

        match select {
            Ok(choice) => {
                match choice {
                    "Search" => request_plugin::search::new(),
                    "Bookmark" => println!("Bookmark"),
                    "Manage Plugin" => println!("Manage Plugin"),
                    "Exit" => {info!("Exiting..."); break;},
                    _ => error!("There was an error, please try again."),
                }
            },
            Err(_) => println!("There was an error, please try again."),
        }
    }
    return Ok(());
}
