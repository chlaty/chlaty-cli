use std::collections::HashMap;

use colored::Colorize;
use inquire::{InquireError, Select};
use tracing::{error};
use clearscreen;
use indicatif::{ProgressBar, ProgressStyle};

use chlaty_core::manage_plugin::{install_plugin, get_plugin_list, get_installed_plugin_list};
use chlaty_core::{manage_plugin::get_installed_plugin_list::PluginInfo};
use chlaty_core::{manage_plugin::install_plugin::PluginManifest};
use crate::display::manage_plugin::install_plugin_type::{PluginDisplay};
use crate::utils::prompt_continue;



pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("failed to clear screen");
    let plugin_list = get_plugin_list::new();
    match plugin_list {
        Ok(plugin_list) => {
            if plugin_list.len() == 0 {
                error!("No plugin installed.");
            }else{
                let get_installed_plugins_result = get_installed_plugin_list::new();

                let mut _installed_plugins: HashMap<String, PluginInfo> = HashMap::new();
                
                match get_installed_plugins_result {
                    Ok(result) => {_installed_plugins = result},
                    Err(e) => {error!("{}", e); return Err(e.into())},
                }

                let options: Vec<PluginDisplay> = plugin_list
                    .iter()
                    .filter_map(|(key, info)| {
                        if _installed_plugins.get(key).is_none() {
                            return Some(PluginDisplay { 
                                id: key.to_string(),
                                title: info.title.to_string(),
                                manifest: info.manifest.to_string()
                            })
                        } else {
                            None
                        }
                    })
                    .collect();

                if options.len() == 0 {
                    println!("{}", "? No plugin available to install.".yellow());
                }else{

                    let select: Result<PluginDisplay, InquireError> = Select::new("Select plugin to install: ", options).prompt();

                    match select {
                        Ok(choice) => {
                            println!("{}", format!("> Downloading plugin ({})...", choice.id).purple());

                            
                            let pb = ProgressBar::new(0);

                            pb.set_style(
                                ProgressStyle::with_template(
                                    "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                                )
                                .unwrap()
                                .progress_chars("#>-"),
                            );

                            match install_plugin::new(&choice.id, "latest", 
                                PluginManifest { 
                                    title: choice.title, manifest: choice.manifest 
                                },
                                move |current, total| {
                                    pb.set_length(total as u64);
                                    pb.set_position(current as u64); 
                                }
                            ) {
                                Ok(_) => println!("{}", "✓ Plugin installed successfully!".green()),
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