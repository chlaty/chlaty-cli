use inquire::{InquireError, Select};
use tracing::{error};
use clearscreen;
use colored::Colorize;

use chlaty_core::manage_plugin::{remove_plugin, get_installed_plugin_list};
use crate::display::manage_plugin::remove_plugin_type::{PluginDisplay};
use crate::utils::prompt_continue;




pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("failed to clear screen");
    let installed_plugins = get_installed_plugin_list::new();
    match installed_plugins {
        Ok(installed_plugins) => {
            if installed_plugins.len() == 0 {
                println!("{}", "? No plugin installed.".yellow());
            }else{

                let options: Vec<PluginDisplay> = installed_plugins
                    .iter()
                    .map(|(key, info)| PluginDisplay { 
                        id: key.to_string(),
                        title: info.title.to_string(),
                        version: info.version.to_string(),
                    })
                    .collect();

                let select: Result<PluginDisplay, InquireError> = Select::new("Select plugin to remove: ", options).prompt();

                match select {
                    Ok(choice) => {
                        println!("{}", format!("> Removing plugin ({})...", choice.id).purple());
                        match remove_plugin::new(&choice.id) {
                            Ok(_) => println!("{}", "✓ Plugin removed successfully!".green()),
                            Err(e) => error!("{}", e),
                        }
                    },
                    Err(e) => error!("{}", e),
                }

            }
        },
        Err(e) => error!("{}", e),
    }

    
    prompt_continue::new();

    return Ok(());
    
}