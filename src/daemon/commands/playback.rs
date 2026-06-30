use crate::player::{Player, Track};
pub fn handle(command: &str, argument: Option<&str>, player: &mut Player) -> Result<String, String>{
    match command {
        "play" => {
            let Some(path) = argument else {
                return Err("No track specified.".to_string());
            };

            let track = Track::new(path.to_string());
            player.play(track);

            Ok(format!("Playing: {path}"))
        } "pause" => {
            player.pause();
            Ok("Playback paused".to_string())
        }

        "resume" => {
            player.resume();
            Ok("Playback resumed".to_string())
        }

        "stop" => {
            player.stop();
            Ok("Playback stopped".to_string())
        }

        "next" => {
            player.next()?;

            let track = player.current_track().ok_or("No current track".to_string())?;
            Ok(format!("Playing: {}",track.source))
        }

        "prev" => {
            player.previous()?;

            let track = player.current_track().ok_or("No current track".to_string())?;
            Ok(format!("Playing: {}",track.source))
        }

        "volume" => {
            let Some(level) = argument else{
                return Err("No volume level specified".to_string());
            };
            let level = level.parse::<u8>().map_err(|_| "Invalid volume level". to_string())?;
            player.set_volume(level);
            Ok(format!("Volume set to {}%", player.get_volume()))
        }

        "play_now" => {
            todo!()
        }

        _ => Err("Unknown playback command.".to_string()),
    }
}