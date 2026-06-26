mod cli;
mod player;
use clap::Parser;
use crate::player::{Player, Track};
use crate::cli::{Cli, Command};

fn main(){
    let cli = Cli::parse();
    let mut player = Player::new();
    match cli.command{
        Command::Play{source} => {
            let track = Track::new(source);
            player.play(track)
        },
        _ => {
          print!("Test")  
        }
    }
}