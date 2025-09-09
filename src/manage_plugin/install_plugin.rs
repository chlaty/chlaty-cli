use inquire::{InquireError, Select};
use tracing::{error, info};
use clearscreen;
use indicatif::{ProgressBar, ProgressStyle};

use chlaty_core::manage_plugin::{download_plugin, get_plugin_list};
use chlaty_core::{manage_plugin::download_plugin::PluginManifest};
use crate::display::manage_plugin::install_plugin_type::{PluginDisplay};
use crate::utils::prompt_continue;



pub fn new() {
    clearscreen::clear().expect("failed to clear screen");
    let installed_plugins = get_plugin_list::new();
    match installed_plugins {
        Ok(installed_plugins) => {
            if installed_plugins.len() == 0 {
                error!("No plugin installed.");
            }else{

                let options: Vec<PluginDisplay> = installed_plugins
                    .iter()
                    .map(|(key, info)| PluginDisplay { 
                        id: key.to_string(),
                        title: info.title.to_string(),
                        manifest: info.manifest.to_string()
                    })
                    .collect();

                let select: Result<PluginDisplay, InquireError> = Select::new("Select plugin to install: ", options).prompt();

                match select {
                    Ok(choice) => {
                        info!("Downloading plugin ({})...", choice.id);

                        
                        let pb = ProgressBar::new(0);

                        pb.set_style(
                            ProgressStyle::with_template(
                                "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                            )
                            .unwrap()
                            .progress_chars("#>-"),
                        );

                        match download_plugin::new(&choice.id, "latest", 
                            PluginManifest { 
                                title: choice.title, manifest: choice.manifest 
                            },
                            move |current, total| {
                                pb.set_length(total as u64); // Update total if needed
                                pb.set_position(current as u64); // Update current progress
                            }
                        ) {
                            Ok(_) => info!("Plugin downloaded successfully!"),
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
    
}