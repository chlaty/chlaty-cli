use std::num::NonZeroUsize;

use inquire::{InquireError, Select, Text};
use tracing::{error};
use tabled::{Table, settings::Style};
use colored::Colorize;
use clearscreen;

use chlaty_core::{request_plugin::get_episode_list};
use crate::display::request_plugin::get_episode_list_type::EpisodeListDisplay;
use crate::utils::prompt_continue;
use crate::request_plugin::{get_episode_server};



pub fn new(plugin_id: &str, id: &str) -> Result<(), Box<dyn std::error::Error>> {

    let result = get_episode_list::new(plugin_id, id);
    match result {
        Ok(result) => {
            let mut page_number:NonZeroUsize = NonZeroUsize::new(1).ok_or("Invalid page number")?;
            loop {
                clearscreen::clear().expect("failed to clear screen");
                let query_vec = result.get(usize::from(page_number) - 1).ok_or("Invalid page number")?;
                let mapped: Vec<EpisodeListDisplay> = query_vec.iter().map(|d| {
                    let mut title = d.title[..d.title.len().min(20)].to_string();
                    if title.len() < d.title.len() {
                        title = format!("{}...", title);
                    }
                    return EpisodeListDisplay {
                        index: d.index.clone(),
                        title: title
                    }
                }).collect();
                let mut table = Table::new(mapped);
                table.with(Style::rounded());

                println!("{}", format!("{}", table).cyan());
                println!("{}", format!("===> [{}/{}] <===", page_number, result.len()).purple());
                let options: Vec<&str> = vec![ "Select episode", "Navigate page", "Exit" ];

                let select: Result<&str, InquireError> = Select::new("Select an option:", options).prompt();

                match select {
                    Ok(choice) => {
                        match choice {
                            "Select episode" => {
                                let input = Text::new("Enter Episode index:").prompt();
                                match input {
                                    Ok(value) => {
                                        let parse_input_index = value.parse::<usize>();
                                        match parse_input_index {
                                            Ok(_) =>  {
                                                let mut select_id: &str = "";
                                                for i in result.iter() {
                                                    for j in i.iter() {
                                                        if j.index == value.parse::<usize>()? {
                                                            select_id = &j.id;
                                                            
                                                            break;
                                                        }
                                                    }   
                                                }

                                                if select_id.is_empty() {
                                                    error!("Unable to find episode from provided index.");
                                                    prompt_continue::new();
                                                }else{
                                                    get_episode_server::new(plugin_id, id, select_id)?;
                                                }
                                            },
                                            Err(e) => error!("{}", e),
                                        }
                                        
                                    },
                                    Err(e) => error!("{}", e),
                                }
                            }
                            "Navigate page" => {
                                let input = Text::new("Enter Page number:").prompt();
                                match input {
                                    Ok(value) => {
                                        let parse_page_number = NonZeroUsize::new(value.parse()?).ok_or("Invalid page number")?;
                                        if parse_page_number > NonZeroUsize::new(result.len()).ok_or("Invalid page number")? {
                                            error!("Page number is out of range.");
                                            prompt_continue::new();
                                        }else{
                                            page_number = parse_page_number;
                                        }
                                    },
                                    Err(e) => error!("{}", e),
                                }
                            }
                            "Exit" => {break;},
                            _ => error!("There was an error, please try again."),
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