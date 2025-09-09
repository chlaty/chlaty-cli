use inquire::{InquireError, Select};
use tracing::{error};


use clearscreen;
use colored::Colorize;

use chlaty_core::manage_plugin::get_installed_plugin_list;

use inquire::{Text};
use tabled::{Table, settings::Style};

mod install_plugin;
mod remove_plugin;

use crate::utils::prompt_continue;
use crate::display::manage_plugin::get_installed_plugin_list_type::InstalledPluginListDisplay;



pub fn main() {
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
                    println!("{}", "> No plugin installed.".yellow());
                }else{
                    let mut table = Table::new(display_installed_plugin_list);
                    table.with(Style::rounded());
                    println!("{}", format!("{}", table).cyan());
                }
                
            },
            Err(err) => println!("{}", err),
        }
        let options: Vec<&str> = vec![ "Install Plugins", "Remove Plugins", "Update Plugins", "Back"];
        let select: Result<&str, InquireError> = Select::new("Select an option: ", options).prompt();

        match select {
            Ok(choice) => {
                match choice {
                    "Install Plugins" => install_plugin::new(),
                    "Remove Plugins" => remove_plugin::new(),
                    "Update Plugins" => println!("Bookmark"),
                    "Back" => {break},
                    _ => error!("There was an error, please try again."),
                }
            },
            Err(_) => println!("There was an error, please try again."),
        }

    }
}
