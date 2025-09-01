use inquire::{InquireError, Select, Text};
use tracing::{error, info};
use colored::Colorize;
use chlaty_core::{request_plugin::search, manage_plugin::get_installed_plugin_list};
use chlaty_core::{manage_plugin::get_installed_plugin_list::SourceInfo};
use std::fmt;
use std::io::{self, Write};

pub struct SourceDisplay<'a> {
    pub key: &'a String,
    pub info: &'a SourceInfo,
}

impl<'a> fmt::Display for SourceDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | v{}", self.info.title, self.info.version)
    }
}


pub fn new() {
    let installed_plugins = get_installed_plugin_list::new();
    match installed_plugins {
        Ok(installed_plugins) => {
            if installed_plugins.len() == 0 {
                error!("No plugin installed.");
            }else{

                let options: Vec<SourceDisplay> = installed_plugins
                    .iter()
                    .map(|(key, info)| SourceDisplay { key, info })
                    .collect();

                let select: Result<SourceDisplay, InquireError> = Select::new("Select plugin: ", options).prompt();

                match select {
                    Ok(choice) => {
                        let selected_plugin_id = choice.key;
                        let input = Text::new("Search:").prompt();
                        match input {
                            Ok(input) => {
                                let result = search::new(&selected_plugin_id, input.as_str(), 1);
                                match result {
                                    Ok(result) => info!("{:?}", result),
                                    Err(err) => error!("{}", err),
                                }
                            },
                            Err(_) => error!("An error happened when asking for your name, try again later."),
                        }
                    },
                    Err(_) => error!("There was an error, please try again."),
                }
            }
        },
        Err(err) => {error!("{}", err)},
    }

    io::stdout().flush().unwrap();
    println!("{}", "Press enter to continue...".yellow());
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();

    
}