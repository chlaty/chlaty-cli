use inquire::{InquireError, Select};
use tracing::{error};
use clearscreen;
use indicatif::{ProgressBar, ProgressStyle};
use colored::Colorize;
use semver::{Version, VersionReq};

use chlaty_core::manage_plugin::{install_plugin, get_installed_plugin_list, get_plugin_list};
use chlaty_core::{manage_plugin::install_plugin::PluginManifest};
use chlaty_core::{manage_plugin::get_plugin_release};
use crate::display::manage_plugin::update_plugin_type::{PluginDisplay};
use crate::utils::prompt_continue;



pub fn new() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("failed to clear screen");
    let plugin_list = get_installed_plugin_list::new();
    match plugin_list {
        Ok(plugin_list) => {
            if plugin_list.len() == 0 {
                println!("{}", "? No plugin installed.".yellow());
            }else{

                let options: Vec<PluginDisplay> = plugin_list
                    .iter()
                    .map(|(key, info)| PluginDisplay { 
                        id: key.to_string(),
                        title: info.title.to_string(),
                        version: info.version.to_string(),
                    })
                    .collect();

                let select: Result<PluginDisplay, InquireError> = Select::new("Select plugin to update: ", options).prompt();

                match select {
                    Ok(choice) => {
                        println!("{}", format!("> Checking update for plugin ({})...", choice.id).purple()); 

                        let plugin_list_result = get_plugin_list::new();

                        let mut plugin_manifest_url: String = String::new();
                        match plugin_list_result {
                            Ok(_plugin_list) => {
                                let plugin = _plugin_list.get(&choice.id).ok_or("Plugin not found")?;
                                plugin_manifest_url = plugin.manifest.to_string();
                            },
                            Err(e) => error!("{}", e),
                        }
                        if !plugin_manifest_url.is_empty() {


                            let plugin_release_result = get_plugin_release::new(&plugin_manifest_url, "latest");

                            let mut update_available: bool = false;


                            match plugin_release_result {
                                Ok(info) => {
                                    let req_ver = VersionReq::parse(&format!(">{}", choice.version))?;
                                    let latest_ver = Version::parse(&info.version)?;
                                    update_available = req_ver.matches(&latest_ver);
                                }
                                Err(e) => error!("[check_plugin_release_result]: {}", e),
                            }

                            if update_available {

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
                                        title: choice.title, manifest: plugin_manifest_url.to_string()
                                    },
                                    move |current, total| {
                                        pb.set_length(total as u64); 
                                        pb.set_position(current as u64); 
                                    }
                                ) {
                                    Ok(_) => println!("{}", "✓ Plugin updated successfully!".green()),
                                    Err(e) => error!("{}", e),
                                }
                            }else{
                                println!("{}", format!("? Plugin already in latest version.").yellow());
                            }
                            
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