use crate::{database::Database, library::Library};

pub fn handle(
    command: &str,
    argument: Option<&str>,
    library: &Library,
    database: &mut Database,
) -> Result<String, String> {
    match command {
        "create" => {
            let name = argument
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .ok_or("No playlist name specified.")?;

            let id = database.create_playlist(name)?;

            Ok(format!("Created playlist '{name}' with ID {id}."))
        }

        "list" => {
            let playlists = database.list_playlists()?;

            if playlists.is_empty() {
                return Ok("No playlists found.".to_string());
            }

            let output = playlists
                .iter()
                .map(|playlist| format!("{}: {}", playlist.id, playlist.name))
                .collect::<Vec<_>>()
                .join("\n");

            Ok(output)
        }

        "show" => {
            let id = argument
                .ok_or("No playlist ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            let playlist = database.get_playlist(id)?.ok_or("Playlist not found.")?;

            if playlist.track_ids.is_empty() {
                return Ok(format!(
                    "{} [{}]\n\nPlaylist is empty.",
                    playlist.name, playlist.id
                ));
            }

            let tracks = playlist
                .track_ids
                .iter()
                .enumerate()
                .map(|(position, track_id)| match library.get(*track_id) {
                    Some(track) => format!(
                        "[{position}] {} - {}",
                        track.metadata.artist.as_deref().unwrap_or("Unknown Artist"),
                        track.display_name()
                    ),
                    None => format!("[{position}] [Missing track — ID {track_id}]"),
                })
                .collect::<Vec<_>>()
                .join("\n");

            Ok(format!("{} [{}]\n\n{}", playlist.name, playlist.id, tracks))
        }

        "add" => {
            let arguments: Vec<&str> = argument
                .ok_or("Playlist ID and at least one track ID required.")?
                .split_whitespace()
                .collect();

            if arguments.len() < 2 {
                return Err(
                    "Usage: playlist add <playlist-id> <track-id> [track-id...]".to_string()
                );
            }

            let playlist_id = arguments[0]
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            if database.get_playlist(playlist_id)?.is_none() {
                return Err("Playlist not found.".to_string());
            }

            let track_ids: Vec<u64> = arguments[1..]
                .iter()
                .map(|id| {
                    id.parse::<u64>()
                        .map_err(|_| format!("Invalid track ID: {id}"))
                })
                .collect::<Result<_, _>>()?;

            // Validate every track before inserting anything.
            for track_id in &track_ids {
                if library.get(*track_id).is_none() {
                    return Err(format!("Track {track_id} not found in library."));
                }
            }

            database.add_tracks_to_playlist(playlist_id, &track_ids)?;

            Ok(format!(
                "Added {} track{} to playlist.",
                track_ids.len(),
                if track_ids.len() == 1 { "" } else { "s" }
            ))
        }

        "remove" => {
            let arguments: Vec<&str> = argument
                .ok_or("Playlist ID and position required.")?
                .split_whitespace()
                .collect();

            if arguments.len() != 2 {
                return Err("Usage: playlist remove <playlist-id> <position>".to_string());
            }

            let playlist_id = arguments[0]
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            let position = arguments[1]
                .parse::<usize>()
                .map_err(|_| "Invalid playlist position.")?;

            if database.get_playlist(playlist_id)?.is_none() {
                return Err("Playlist not found.".to_string());
            }

            let removed = database.remove_track_from_playlist(playlist_id, position)?;

            if !removed {
                return Err("Playlist position not found.".to_string());
            }

            Ok(format!(
                "Removed track at position {position} from playlist."
            ))
        }
        "delete" => {
            let id = argument
                .ok_or("No playlist ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            let deleted = database.delete_playlist(id)?;

            if !deleted {
                return Err("Playlist not found.".to_string());
            }

            Ok(format!("Deleted playlist {id}."))
        }
        "rename" => {
            let arguments: Vec<&str> = argument
                .ok_or("Playlist ID and new name required.")?
                .splitn(2, ' ')
                .collect();

            if arguments.len() != 2 {
                return Err("Usage: playlist rename <playlist-id> <name>".to_string());
            }

            let id = arguments[0]
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            let name = arguments[1].trim();

            if name.is_empty() {
                return Err("Playlist name cannot be empty.".to_string());
            }

            let renamed = database.rename_playlist(id, name)?;

            if !renamed {
                return Err("Playlist not found.".to_string());
            }

            Ok(format!("Renamed playlist {id} to '{name}'."))
        }
        "remove_all" => {
            let playlist_id = argument
                .ok_or("No playlist ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            if database.get_playlist(playlist_id)?.is_none() {
                return Err("Playlist not found.".to_string());
            }

            let removed = database.clear_playlist(playlist_id)?;

            Ok(format!("Removed {removed} tracks from playlist."))
        }

        "remove_missing" => {
            let playlist_id = argument
                .ok_or("No playlist ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;
            if database.get_playlist(playlist_id)?.is_none() {
                return Err("Playlist not found.".to_string());
            }
            let removed = database.remove_missing_tracks_in_playlist(playlist_id, library)?;
            Ok(format!("Removed {removed} missing track(s)."))
        }

        "move" => {
            let arguments: Vec<&str> = argument
                .ok_or("Playlist ID, source position, and destination position required.")?
                .split_whitespace()
                .collect();

            if arguments.len() != 3 {
                return Err(
                    "Usage: playlist move <playlist-id> <from-position> <to-position>".to_string(),
                );
            }

            let playlist_id = arguments[0]
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            let from = arguments[1]
                .parse::<usize>()
                .map_err(|_| "Invalid source position.")?;

            let to = arguments[2]
                .parse::<usize>()
                .map_err(|_| "Invalid destination position.")?;

            let moved = database.move_track_in_playlist(playlist_id, from, to)?;

            if !moved {
                return Err("Playlist position out of bounds.".to_string());
            }

            Ok(format!("Moved track from position {from} to {to}."))
        }
        "info" => {
            let id = argument
                .ok_or("No playlist ID specified.")?
                .parse::<u64>()
                .map_err(|_| "Invalid playlist ID.")?;

            let playlist = database.get_playlist(id)?.ok_or("Playlist not found.")?;

            let total = playlist.track_ids.len();

            let available = playlist
                .track_ids
                .iter()
                .filter(|track_id| library.get(**track_id).is_some())
                .count();

            let missing = total - available;

            let total_duration = playlist
                .track_ids
                .iter()
                .filter_map(|track_id| library.get(*track_id))
                .filter_map(|track| track.metadata.duration)
                .sum::<std::time::Duration>();

            let hours = total_duration.as_secs() / 3600;
            let minutes = (total_duration.as_secs() % 3600) / 60;
            let seconds = total_duration.as_secs() % 60;

            Ok(format!(
                "ID: {}\n\
                Name: {}\n\
                Tracks: {}\n\
                Available: {}\n\
                Missing: {}\n\
                Duration: {:02}:{:02}:{:02}",
                playlist.id, playlist.name, total, available, missing, hours, minutes, seconds,
            ))
        }
        _ => Err("Unknown playlist command.".to_string()),
    }
}
