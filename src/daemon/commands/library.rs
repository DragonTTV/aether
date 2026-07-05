use std::path::Path;

use crate::library::{scanner, Library, storage};

pub fn handle(
    command: &str,
    argument: Option<&str>,
    library: &mut Library,
) -> Result<String, String> {
    match command {
        "scan" => {
            let Some(path) = argument else {
                return Err("No path specified.".to_string());
            };

            library.clear();

            scanner::scan(Path::new(path), library)?;
            storage::save(library)?;
            Ok(format!("Scanned {} tracks.", library.len()))
        }

        "list" => {
            if library.is_empty() {
                return Ok("Library is empty.".to_string());
            }

            let list = library
                .tracks()
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
        

        _ => Err("Unknown library command.".to_string()),
    }
}