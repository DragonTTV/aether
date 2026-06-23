mod cli;
use clap::Parser;

use crate::cli::{Cli, Command};

fn main(){
    let cli = Cli::parse();
    match cli.command{
        Command::Play{
            source
        } => {
            println!("Playing: {}", source);
        },
        _ => {
          print!("Test")  
        }
    }
}