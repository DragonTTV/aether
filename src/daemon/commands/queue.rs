// use std::fmt::format;

use crate::player::{Player, Track};
pub fn handle(command: &str, argument: Option<&str>, player: &mut Player,) -> Result<String, String>{
    match command {
        "add" => {
            let Some(path) = argument else{
                return Err("No tracks specified".to_string());
            };
            let track = Track::new(path.to_string());
            player.play(track);

            Ok(format!("Added to queue: {path}"))
        }

        "remove" => {
            let Some(index) = argument else{
                return Err("No index specified.".to_string());
            };
            let index = index.parse::<usize>().map_err(|_|"Invalid index".to_string())?; 
            player.remove_from_queue(index)?;
            Ok(format!("Removed track {index}"))
            
        }

        "list" => {
            let tracks = player.queue().tracks();
            if tracks.is_empty(){
                return Ok("Queue is empty".to_string());
            }
            let list = tracks
                .iter()
                .enumerate()
                .map(|(i, track)| format!("{i}: {}", track.source))
                .collect::<Vec<_>>()
                .join("\n");
            Ok(list)
        }

        "clear" => {
            player.clear_queue();
            Ok("Queue cleared.".to_string())
        }

        _ => Err("Unknown queue command.".to_string()),
    }
}