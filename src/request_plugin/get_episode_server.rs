use inquire::{InquireError, Select, Text};
use tracing::{error};
use tabled::{ Table, settings::Style};
use colored::Colorize;

use chlaty_core::{request_plugin::get_episode_server};
use crate::display::request_plugin::get_episode_server_type::EpisodeServerDisplay;
use crate::utils::prompt_continue;
use crate::request_plugin::{get_server};
use clearscreen;


pub fn new(plugin_id: &str, content_id:&str, id: &str) {

    let result = get_episode_server::new(plugin_id, id);
    match result {
        Ok(result) => {
            loop {
                clearscreen::clear().expect("failed to clear screen");
                let mut count = 1;
                for server_type in result.keys() {
                    let query_vec = result.get(server_type).unwrap();
                    let mapped: Vec<EpisodeServerDisplay> = query_vec.iter().map(|d| {
                        let new_display = EpisodeServerDisplay {
                            index: count,
                            title: &d.title,
                            id: &d.id
                        };
                        count += 1;
                        return new_display;
                    }).collect();
                    println!("{}", format!("> {}", server_type).purple());
                    let mut table = Table::new(mapped);
                    table.with(Style::rounded());

                    println!("{}", format!("{}", table).cyan());
                }
                
                let options: Vec<&str> = vec![ "Select server", "Back" ];

                let select: Result<&str, InquireError> = Select::new("Select an option:", options).prompt();

                match select {
                    Ok(choice) => {
                        match choice {
                            "Select server" => {
                                let input = Text::new("Enter Server index:").prompt();
                                match input {
                                    Ok(value) => {
                                        let parse_input_index = value.parse::<usize>();
                                        match parse_input_index {
                                            Ok(index) =>  {
                                                let mut select_id: &str = "";

                                                let mut count: usize = 1;
                                                for i in result.keys() {
                                                    for j in result.get(i).unwrap() {
                                                        if count == index {
                                                            select_id = &j.id;
                                                            break;
                                                        }
                                                        count += 1;
                                                    }   
                                                    if !select_id.is_empty() {
                                                        break;
                                                    }
                                                }

                                                if select_id.is_empty() {
                                                    error!("Unable to find server from provided index.");
                                                    prompt_continue::new();
                                                }else{
                                                    get_server::new(plugin_id, content_id, id, select_id);
                                                }
                                            },
                                            Err(e) => error!("{}", e),
                                        }
                                        
                                    },
                                    Err(e) => error!("{}", e),
                                }
                            }
                            "Back" => {
                                clearscreen::clear().expect("failed to clear screen");
                                break;
                            },
                            _ => error!("There was an error, please try again."),
                        }
                    },
                    Err(e) => error!("{}", e),
                }
            }

            
        },
        Err(e) => error!("{}", e),
    }
        
    
}