use std::num::NonZeroUsize;

use inquire::{InquireError, Select, Text};
use tracing::{error, warn};
use colored::Colorize;


use chlaty_core::{request_plugin::search, manage_plugin::get_installed_plugin_list};
use crate::display::request_plugin::search_type::{PluginDisplay, SearchDisplay};
use crate::utils::prompt_continue;
use crate::request_plugin::{get_episode_list};



pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    let installed_plugins = get_installed_plugin_list::new();
    match installed_plugins {
        Ok(installed_plugins) => {
            if installed_plugins.len() == 0 {
                println!("{}", "? No plugin installed.".yellow());
            }else{

                let options: Vec<PluginDisplay> = installed_plugins
                    .iter()
                    .map(|(key, info)| PluginDisplay { key, info })
                    .collect();

                let select: Result<PluginDisplay, InquireError> = Select::new("Select plugin: ", options).prompt();

                let mut selected_plugin_id: &str = "";
                match select {
                    Ok(choice) => {
                        selected_plugin_id = choice.key;
                    },
                    Err(e) => error!("{}", e),
                }

                let mut search_input: String = String::new();
                if !selected_plugin_id.is_empty() {
                    let input = Text::new("Search:").prompt();
                    match input {
                        Ok(value) => {
                            search_input = value;
                        },
                        Err(e) => error!("{}", e),
                    }
                }

                let mut page_number_input: NonZeroUsize = NonZeroUsize::new(1).unwrap();
                if !search_input.is_empty() {
                    let input = Text::new("Page number:").prompt();
                    match input {
                        Ok(value) => {
                            match value.parse::<NonZeroUsize>(){
                                Ok(parsed_value) => {page_number_input = parsed_value;},
                                Err(_) => warn!("Invalid page number. Falling back to 1."),
                            }
                            
                        },
                        Err(e) => error!("{}", e),
                    }
                }

                if !search_input.is_empty() {
                    let result = search::new(selected_plugin_id, &search_input, page_number_input);
                    match result {
                        Ok(result) => {
                            let options: Vec<SearchDisplay> = result
                                .iter()
                                .map(|i| SearchDisplay { id: &i.id, title: &i.title })
                                .collect();

                            let select: Result<SearchDisplay, InquireError> = Select::new("Pick content: ", options).prompt();

                            match select {
                                Ok(choice) => {
                                    clearscreen::clear().expect("failed to clear screen");
                                    println!("{}", format!("> {}", &choice.title).purple());
                                    get_episode_list::new(selected_plugin_id, &choice.id)?;
                                    return Ok(());
                                },
                                Err(e) => error!("{}", e),
                            }
                            
                        },
                        Err(e) => error!("{}", e),
                    }
                }

            }
        },
        Err(e) => error!("{}", e),
    }

    
    prompt_continue::new();

    return Ok(());
    
}