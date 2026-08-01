
use crate::library::Library;
use crate::player::{Player, RepeatMode, Track};
use crate::database::Database;

pub fn handle(
    command: &str,
    argument: Option<&str>,
    player: &mut Player,
    library: &Library,
    database: &Database,
) -> Result<String, String> {
    match command {
        "play" => {
            let Some(path) = argument else {
                return Err("No track specified.".to_string());
            };

            let track = Track::new(path.to_string());
            let track_name = track.display_name().to_string();
            let outcome = player.play(track);
            if outcome.started_playing {
                Ok(format!("Now Playing: {track_name}"))
            } else {
                Ok(format!("Added to queue: {track_name}"))
            }
        }
        "pause" => match player.pause() {
            Ok(()) => Ok("Playback paused".to_string()),
            Err(e) => Ok(e.to_string()),
        },

        "resume" => match player.resume() {
            Ok(()) => Ok("Playback resumed".to_string()),
            Err(e) => Ok(e.to_string()),
        },

        "stop" => match player.stop() {
            Ok(()) => Ok("Playback stopped".to_string()),
            Err(e) => Ok(e.to_string()),
        },

        "next" => match player.next_track() {
            Ok(track) => Ok(format!(
                "Skipped to next track.\n\nNow playing: {}",
                track.display_name()
            )),

            Err(e) => Ok(e.to_string()),
        },

        "prev" => match player.previous_track() {
            Ok(track) => Ok(format!(
                "Returned to previous track.\n\nNow playing: {}",
                track.display_name()
            )),

            Err(e) => Ok(e.to_string()),
        },

        "volume" => {
            let Some(level) = argument else {
                return Err("No volume level specified".to_string());
            };
            let level = level
                .parse::<u8>()
                .map_err(|_| "Invalid volume level".to_string())?;
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

            let track = library.get(id).ok_or("Track not found.")?;

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

            let track = library.get(id).ok_or("Track not found.")?;

            player.play_now(track.clone());

            Ok(format!("Playing {}", track.display_name()))
        }
        "play_playlist" | "play_now_playlist" => {
            let id = argument
                .ok_or("No playlist ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            let playlist = database
                .get_playlist(id)?
                .ok_or("Playlist not found.")?;

            let tracks: Vec<Track> = playlist
                .track_ids
                .iter()
                .filter_map(|track_id| library.get(*track_id).cloned())
                .collect();

            if tracks.is_empty() {
                return Err("Playlist contains no available tracks.".to_string());
            }

            let track_count = tracks.len();
            let playlist_name = playlist.name;

            if command == "play_now_playlist" {
                player.play_all_now(tracks);

                Ok(format!(
                    "Playing playlist '{playlist_name}' ({track_count} tracks)."
                ))
            } else {
                let outcome = player.play_all(tracks);

                if outcome.started_playing {
                    Ok(format!(
                        "Playing playlist '{playlist_name}' ({track_count} tracks)."
                    ))
                } else {
                    Ok(format!(
                        "Added playlist '{playlist_name}' to queue ({track_count} tracks)."
                    ))
                }
            }
        }
        "repeat" => {
            match argument {
                None => Ok(format!("Repeat: {}", player.repeat())),
                Some("off") => {
                    player.set_repeat(RepeatMode::Off);
                    Ok("Repeat mode set to Off.".into())
                }
                Some("track") => {
                    player.set_repeat(RepeatMode::Track);
                    Ok("Repeat mode set to Track.".into())
                }
                Some("queue") => {
                    player.set_repeat(RepeatMode::Queue);
                    Ok("Repeat mode set to Queue.".into())
                }
                Some(_) => {
                    Err("Invalid repeat mode.".into())
                }
            }
        }
        _ => Err("Unknown playback command.".to_string()),
    }
}
