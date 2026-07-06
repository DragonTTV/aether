use crate::player::{Player, Track};
use crate::library::{Library};
pub fn handle(command: &str, argument: Option<&str>, player: &mut Player, library: &Library,) -> Result<String, String>{
    match command {
        "play" => {
            let Some(path) = argument else {
                return Err("No track specified.".to_string());
            };

            let track = Track::new(path.to_string());
            let track_name = track.display_name().to_string();
            let outcome = player.play(track);
            if outcome.started_playing{
                Ok(format!("Now Playing: {track_name}"))
            }else{
                Ok(format!("Added to queue: {track_name}"))
            }

        } "pause" => {
            match player.pause(){
                Ok(()) => {Ok("Playback paused".to_string())},
                Err(e) => Ok(e.to_string()),
            }
        }

        "resume" => {
            match player.resume() {
                Ok(()) => Ok("Playback resumed".to_string()),
                Err(e) => Ok(e.to_string()),
            }
        }

        "stop" => {
            match player.stop(){
                Ok(()) => Ok("Playback stopped".to_string()),
                Err(e) => Ok(e.to_string()),
            }
        }

        "next" => {
            match player.next() {
            Ok(track) => Ok(format!(
                "Skipped to next track.\n\nNow playing: {}",
                track.display_name()
            )),

            Err(e) => Ok(e.to_string()),
            }
        }

        "prev" => {
            match player.previous() {
                Ok(track) => Ok(format!(
                    "Returned to previous track.\n\nNow playing: {}",
                    track.display_name()
                )),

                Err(e) => Ok(e.to_string()),
            }
        }

        "volume" => {
            let Some(level) = argument else{
                return Err("No volume level specified".to_string());
            };
            let level = level.parse::<u8>().map_err(|_| "Invalid volume level".to_string())?;
            player.set_volume(level);
            Ok(format!("Volume set to {}%", player.get_volume()))
        }

        "play_now" => {
            let source = argument.ok_or("No source specified.")?;

            let track = Track::new(source.to_string());

            player.play_now(track);

            Ok("Playing immediately.".into())
        }
        "play_id" => {
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
        "play_now_id" => {
            let id = argument
                .ok_or("No track ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid track ID.")?;

            let track = library
                .get(id)
                .ok_or("Track not found.")?;

            player.play_now(track.clone());

            Ok(format!("Playing {}", track.display_name()))
        }
        _ => Err("Unknown playback command.".to_string()),
    }
}