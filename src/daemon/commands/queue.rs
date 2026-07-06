// use std::fmt::format;

use crate::{library::Library, player::{Player, Track}};
pub fn handle(command: &str, argument: Option<&str>, player: &mut Player, library: &mut Library) -> Result<String, String>{
    match command {
        "add" => {
            let Some(path) = argument else{
                return Err("No tracks specified".to_string());
            };
            let track = Track::new(path.to_string());
            let track_name = track.display_name().to_string();
            player.play(track);

            Ok(format!("Added to queue: {track_name}"))
        }

        "remove" => {
            let Some(index) = argument else {
                return Err("No queue index specified.".to_string());
            };

            let index = index
                .parse::<usize>()
                .map_err(|_| "Invalid queue index.".to_string())?;

            let track_name = player
                .queue
                .tracks()
                .get(index)
                .map(|track| track.display_name().to_string());

            player.queue.remove(index)?;

            Ok(match track_name {
                Some(name) => format!("Removed from queue: {}", name),
                None => "Track removed.".to_string(),
            })
        }

        "list" => {
            let tracks = player.queue().tracks();
            let current = player.queue().current_index();

            if tracks.is_empty() {
                return Ok("Queue is empty.".to_string());
            }

            let mut output = String::new();

            output.push_str("Queue\n");
            output.push_str("-----\n\n");

            for (i, track) in tracks.iter().enumerate() {
                let marker = if Some(i) == current { "▶" } else { "" };

                output.push_str(&format!(
                    "{marker} [{i}] {}\n",
                    track.display_name()
                ));
            }

            Ok(output)
        }

        "clear" => {
            player.clear_queue();
            Ok("Queue cleared.".to_string())
        }
        "add_id" => {
            let id = argument
                .ok_or("No track ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid track ID.")?;

            let track = library
                .get(id)
                .ok_or("Track not found.")?;

            let outcome = player.play(track.clone());

            if outcome.started_playing {
                Ok(format!("Playing {}", track.display_name()))
            } else {
                Ok(format!("Added {} to queue", track.display_name()))
            }
        }
        _ => Err("Unknown queue command.".to_string()),
    }
}