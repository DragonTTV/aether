mod cli;
mod player;
mod audio;
use clap::Parser;
use crate::player::{Player, Track};
use crate::cli::{Cli, Command};
use std::io;
fn main(){
    let cli = Cli::parse();
    let mut player = Player::new();
    match cli.command{
        Command::Play{source} => {
            let track = Track::new(source);
            player.play(track);

            println!("Press Enter to exit...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        },
        _ => {
          print!("Test")  
        }
    }
}