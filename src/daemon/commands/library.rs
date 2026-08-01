use std::path::Path;
use std::collections::{HashMap, HashSet};
use crate::player::Track;
use crate::{
    cli::SortBy,
    database::Database,
    library::{Library, scanner},
};

pub fn handle(
    command: &str,
    argument: Option<&str>,
    library: &mut Library,
    database: &mut Database,
) -> Result<String, String> {
    match command {
        "scan" => {
            let Some(path) = argument else {
                return Err("No path specified.".to_string());
            };

            library.clear();

            scanner::scan(Path::new(path), library)?;
            database.save_library(library)?;
            Ok(format!("Scanned {} tracks.", library.len()))
        }

        "list" => {
            if library.is_empty() {
                return Ok("Library is empty.".to_string());
            }

            let sort = match argument {
                Some("title") => Some(SortBy::Title),
                Some("artist") => Some(SortBy::Artist),
                Some("album") => Some(SortBy::Album),
                Some(_) => return Err("Unknown sort option.".into()),
                None => None,
            };

            let tracks = library.sorted_tracks(sort);
            let list = tracks
                .iter()
                .map(|track| {
                    format!(
                        "{}: {} - {}",
                        track.id,
                        track.metadata.artist.as_deref().unwrap_or("Unknown Artist"),
                        track.display_name()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            Ok(list)
        }
        "search" => {
            let Some(query) = argument else {
                return Err("No search query specified.".into());
            };

            let results = library.search(query);

            if results.is_empty() {
                return Ok("No matching tracks found.".into());
            }

            let list = results
                .iter()
                .map(|track| {
                    format!(
                        "{}: {} - {}",
                        track.id,
                        track.metadata.artist.as_deref().unwrap_or("Unknown Artist"),
                        track.display_name(),
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            Ok(list)
        }
        "info" => {
            let id = argument
                .ok_or("No track ID specified")?
                .parse::<u64>()
                .map_err(|_| "Invalid track ID.")?;
            let track = library.get(id).ok_or("Track not found.")?;
            let metadata = &track.metadata;
            Ok(format!(
                "ID: {}\n\
                Title: {}\n\
                Artist: {}\n\
                Album: {}\n\
                Duration: {}\n\
                Release: {}\n\
                Path: {}",
                track.id,
                track.display_name(),
                metadata.artist.as_deref().unwrap_or("Unknown Artist"),
                metadata.album.as_deref().unwrap_or("Unknown Album"),
                track
                    .formatted_duration()
                    .unwrap_or_else(|| "Unknown".into()),
                metadata.release_date.as_deref().unwrap_or("Unknown"),
                track.source,
            ))
        }
        "rescan" => {
            let paths = library.scan_paths().to_vec();
            let scanned_dirs = paths.len();

            if paths.is_empty() {
                return Err("No scan paths found.".into());
            }

            let old_tracks: HashMap<String, Track> = library
                .tracks()
                .iter()
                .map(|track| (track.source.clone(), track.clone()))
                .collect();
            
            let old_sources: HashSet<String> = old_tracks.keys().cloned().collect();

            library.clear_tracks();

            for path in paths {
                scanner::rescan(Path::new(&path), library, &old_tracks)?;
            }
            let new_sources: HashSet<String> = library
                .tracks()
                .iter()
                .map(|track| track.source.clone())
                .collect();

            let added = new_sources.difference(&old_sources).count();
            let removed = old_sources.difference(&new_sources).count();
            database.save_library(library)?;

            let new_tracks = library.len();

            let message = if added == 0 && removed == 0 {
                format!(
                    "Library rescan complete.\n\
                    Directories scanned: {}\n\
                    Tracks found: {}\n\
                    No changes detected.",
                    scanned_dirs, new_tracks
                )
            } else {
                format!(
                    "Library rescan complete.\n\
                    Directories scanned: {}\n\
                    Tracks found: {}\n\
                    Changes:\n\
                    +{} added\n\
                    -{} removed",
                    scanned_dirs, new_tracks, added, removed
                )
            };

            Ok(message)
        }
        "rescan_reid" => {
            let paths = library.scan_paths().to_vec();
            let scanned_dirs = paths.len();

            if paths.is_empty() {
                return Err("No scan paths found.".into());
            }

            let old_tracks: HashMap<String, Track> = library
                .tracks()
                .iter()
                .map(|track| (track.source.clone(), track.clone()))
                .collect();

            let old_sources: HashSet<String> = old_tracks.keys().cloned().collect();

            library.clear();

            let mut id_map = HashMap::new();

            for path in paths {
                let map = scanner::rescan_reid(
                    Path::new(&path),
                    library,
                    &old_tracks,
                )?;

                id_map.extend(map);
            }

            database.update_playlist_track_ids(&id_map)?;

            let new_sources: HashSet<String> = library
                .tracks()
                .iter()
                .map(|track| track.source.clone())
                .collect();

            let added = new_sources.difference(&old_sources).count();
            let removed = old_sources.difference(&new_sources).count();

            database.save_library(library)?;

            let new_tracks = library.len();

            let message = if added == 0 && removed == 0 {
                format!(
                    "Library rescan complete.\n\
                    Directories scanned: {}\n\
                    Tracks found: {}\n\
                    Track IDs reassigned.\n\
                    No changes detected.",
                    scanned_dirs, new_tracks
                )
            } else {
                format!(
                    "Library rescan complete.\n\
                    Directories scanned: {}\n\
                    Tracks found: {}\n\
                    Track IDs reassigned.\n\
                    Changes:\n\
                    +{} added\n\
                    -{} removed",
                    scanned_dirs, new_tracks, added, removed
                )
            };

            Ok(message)
        }
        _ => Err("Unknown library command.".to_string()),
    }
}
