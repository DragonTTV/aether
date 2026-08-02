
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::library::Library;
use crate::mpris::server::MprisServer;
use crate::player::{Player, RepeatMode, Track};
use crate::database::Database;

pub async fn handle(
    command: &str,
    argument: Option<&str>,
    player: &Arc<Mutex<Player>>,
    library: &Library,
    database: &Database,
    mpris: &MprisServer
) -> Result<String, String> {
    match command {
        "play" => {
            let Some(path) = argument else {
                return Err("No track specified.".to_string());
            };

            let track = Track::new(path.to_string());
            let track_name = track.display_name().to_string();
            let outcome = {
                let mut player = player.lock().unwrap();
                player.play(track)
            };
            if outcome.started_playing {
                let _ = mpris.notify_metadata().await;
                let _ = mpris.notify_can_play_pause().await;
                let _ = mpris.notify_playback_status().await;
                let _ = mpris.notify_position().await;

                Ok(format!("Now Playing: {track_name}"))

            } else {
                Ok(format!("Added to queue: {track_name}"))
            }
        }
        "pause" => {
            let result = {
                let mut player = player.lock().unwrap();
                player.pause()
            };
            match result{
                Ok(()) => {
                    let _ = mpris.notify_playback_status().await;
                    Ok("Playback paused".to_string())
                },
                Err(e) => Ok(e.to_string()),
            }
        }

        "resume" => {
            let result = {
                let mut player = player.lock().unwrap();
                player.resume()
            };
            match result {
                Ok(()) => {
                    let _ = mpris.notify_playback_status().await;
                    Ok("Playback resumed".to_string())
                },
                Err(e) => Ok(e.to_string()),
            }
        }

        "stop" => {
            let result = {
                let mut player = player.lock().unwrap();
                player.stop()
            };
            match result {
                Ok(()) => {
                    let _ = mpris.notify_playback_status().await;
                    let _ = mpris.notify_position().await;
                    
                    Ok("Playback stopped".to_string())
                },
                Err(e) => Ok(e.to_string()),
            }
        }

        "next" => {
            
            let result = {
                let mut player = player.lock().unwrap();
                player.next_track().cloned()
            };
            match result {
                Ok(track) => {
                    let _ = mpris.notify_metadata().await;
                    let _ = mpris.notify_can_play_pause().await;
                    let _ = mpris.notify_playback_status().await;
                    let _ = mpris.notify_position().await;
                    Ok(format!("Skipped to next track.\n\nNow playing: {}",track.display_name()))
                },
                Err(e) => Ok(e.to_string()),
            }
        }
            
        "prev" => {
            let result = {
                let mut player = player.lock().unwrap();
                player.previous_track().cloned()
            };

            match result {
                Ok(track) => {
                    let _ = mpris.notify_metadata().await;
                    let _ = mpris.notify_can_play_pause().await;
                    let _ = mpris.notify_playback_status().await;
                    let _ = mpris.notify_position().await;

                    Ok(format!(
                        "Returned to previous track.\n\nNow playing: {}",
                        track.display_name()
                    ))
                }
                Err(e) => Ok(e.to_string()),
            }
        }

        "volume" => {
            let Some(level) = argument else {
                return Err("No volume level specified".to_string());
            };
            let level = level
                .parse::<u8>()
                .map_err(|_| "Invalid volume level".to_string())?;
            let volume = {
                let mut player = player.lock().unwrap();
                player.set_volume(level);
                player.get_volume()
            };
            Ok(format!("Volume set to {}%", volume))
        }

        "play_now" => {
            let source = argument.ok_or("No source specified.")?;

            let track = Track::new(source.to_string());

            {
                let mut player = player.lock().unwrap();
                player.play_now(track);
            } // MutexGuard dropped here

            let _ = mpris.notify_metadata().await;
            let _ = mpris.notify_can_play_pause().await;
            let _ = mpris.notify_playback_status().await;
            let _ = mpris.notify_position().await;

            Ok("Playing immediately.".into())
        }

        "play_id" => {
            let id = argument
                .ok_or("No track ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid track ID.")?;

            let track = library.get(id).ok_or("Track not found.")?;

            let outcome = {
                let mut player = player.lock().unwrap();
                player.play(track.clone())
            };

            if outcome.started_playing {
                let _ = mpris.notify_metadata().await;
                let _ = mpris.notify_can_play_pause().await;
                let _ = mpris.notify_playback_status().await;
                let _ = mpris.notify_position().await;
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

            {
                let mut player = player.lock().unwrap();
                player.play_now(track.clone());
            }
            let _ = mpris.notify_metadata().await;
            let _ = mpris.notify_can_play_pause().await;
            let _ = mpris.notify_playback_status().await;
            let _ = mpris.notify_position().await;
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
                {
                    let mut player = player.lock().unwrap();
                    player.play_all_now(tracks);
                }
                let _ = mpris.notify_metadata().await;
                let _ = mpris.notify_can_play_pause().await;
                let _ = mpris.notify_playback_status().await;
                let _ = mpris.notify_position().await;
                Ok(format!(
                    "Playing playlist '{playlist_name}' ({track_count} tracks)."
                ))
            } else {
                let outcome = {
                    let mut player = player.lock().unwrap();
                    player.play_all(tracks)
                };

                if outcome.started_playing {
                    let _ = mpris.notify_metadata().await;
                    let _ = mpris.notify_can_play_pause().await;
                    let _ = mpris.notify_playback_status().await;
                    let _ = mpris.notify_position().await;
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
                None => {
                    let repeat = {
                        let player = player.lock().unwrap();
                        player.repeat()
                    };

                    Ok(format!("Repeat: {}", repeat))
                }
                Some("off") => {
                    {
                        let mut player = player.lock().unwrap();
                        player.set_repeat(RepeatMode::Off);
                    }
                    Ok("Repeat mode set to Off.".into())
                }
                Some("track") => {
                    {
                        let mut player = player.lock().unwrap();
                        player.set_repeat(RepeatMode::Track);
                    }
                    Ok("Repeat mode set to Track.".into())
                }
                Some("queue") => {
                    {
                        let mut player = player.lock().unwrap();
                        player.set_repeat(RepeatMode::Queue);
                    }
                    Ok("Repeat mode set to Queue.".into())
                }
                Some(_) => {
                    Err("Invalid repeat mode.".into())
                }
            }
        }
        "shuffle" => {
            match argument {
                None => {
                    let guard = player.lock().unwrap();
                    Ok(shuffle_status(&guard))
                },

                Some("on") => {
                    {
                        let mut player = player.lock().unwrap();
                        player.set_shuffle(true);
                    }
                    let guard = player.lock().unwrap();
                    Ok(shuffle_status(&guard))
                }

                Some("off") => {
                    {
                        let mut player = player.lock().unwrap();
                        player.set_shuffle(false);
                    };
                    let guard = player.lock().unwrap();
                    Ok(shuffle_status(&guard))
                }

                Some(_) => Err("Invalid shuffle mode.".into()),
            }
        }
        "seek" => {
            let seconds = argument
                .ok_or("No seek position specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid seek position.")?;

            {
                let player = player.lock().unwrap();
                player
                    .seek(Duration::from_secs(seconds))
                    .map_err(|e| e.to_string())?;
            }
            let _ = mpris.notify_position().await;
            Ok(format!("Seeked to {}s.", seconds))
        }
        _ => Err("Unknown playback command.".to_string()),
    }
}

fn shuffle_status(player: &Player) -> String {
    let mut response = String::new();

    response.push_str(&format!(
        "Shuffle: {}\n",
        if player.shuffle() { "On" } else { "Off" }
    ));

    if player.shuffle() {
        response.push_str("\nPlayback Order\n\n");

        let current = player.queue().current_index();

        for &index in player.queue().shuffle_order() {
            let track = &player.queue().tracks()[index];

            if Some(index) == current {
                response.push_str(&format!("▶ {}\n", track.display_name()));
            } else {
                response.push_str(&format!("  {}\n", track.display_name()));
            }
        }
    }

    response
}