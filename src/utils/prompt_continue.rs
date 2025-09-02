use std::io::{self, Write};
use colored::Colorize;

pub fn new() {
    io::stdout().flush().unwrap();
    println!("{}", "Press enter to continue...".yellow());
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}