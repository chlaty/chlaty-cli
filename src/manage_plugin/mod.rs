use inquire::{InquireError, Select};
use tracing::{error};


use clearscreen;
use colored::Colorize;

use chlaty_core::manage_plugin::get_installed_plugin_list;

use tabled::{Table, settings::Style};

mod install_plugin;
mod remove_plugin;
mod update_plugin;
mod update_all_plugin;

use crate::display::manage_plugin::get_installed_plugin_list_type::InstalledPluginListDisplay;



pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        clearscreen::clear().expect("failed to clear screen");

        let get_installed_plugin_list_result = get_installed_plugin_list::new();

        let mut display_installed_plugin_list: Vec<InstalledPluginListDisplay> = vec![];

        match get_installed_plugin_list_result {
            Ok(list) => {
                for plugin_key in list.keys() {
                    display_installed_plugin_list.push(InstalledPluginListDisplay {
                        id: plugin_key.to_string(),
                        title: list.get(plugin_key).unwrap().title.to_string(),
                        version: list.get(plugin_key).unwrap().version.to_string(),
                    });
                }
                if display_installed_plugin_list.len() == 0 {
                    println!("{}", "? No plugin installed.".yellow());
                }else{
                    println!("{}", "> Installed plugins.".purple());
                    let mut table = Table::new(display_installed_plugin_list);
                    table.with(Style::rounded());
                    println!("{}", format!("{}", table).cyan());
                }
                
            },
            Err(err) => println!("{}", err),
        }
        let options: Vec<&str> = vec![ "Install plugin", "Remove plugin", "Update plugin", "Update all plugin", "Back"];
        let select: Result<&str, InquireError> = Select::new("Select an option: ", options).prompt();

        match select {
            Ok(choice) => {
                match choice {
                    "Install plugin" => install_plugin::new()?,
                    "Remove plugin" => remove_plugin::new()?,
                    "Update plugin" => update_plugin::new()?,
                    "Update all plugin" => update_all_plugin::new()?,
                    "Back" => {
                        return Ok(())
                    },
                    _ => error!("There was an error, please try again."),
                }
            },
            Err(_) => error!("There was an error, please try again."),
        }

    }
}
