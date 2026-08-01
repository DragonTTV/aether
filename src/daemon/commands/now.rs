use crate::player::Player;

pub fn now(player: &Player) -> Result<String, String> {
    let status = player.status();

    let mut output = String::new();

    match status.current_track {
        Some(track) => {
            output.push_str("Now Playing\n");
            output.push_str("-----------\n\n");

            // Title
            output.push_str(track.display_name());
            output.push('\n');

            // Artist • Album
            match (
                track.metadata.artist.as_deref(),
                track.metadata.album.as_deref(),
            ) {
                (Some(artist), Some(album)) => {
                    output.push_str(&format!("{artist} • {album}\n"));
                }
                (Some(artist), None) => {
                    output.push_str(&format!("{artist}\n"));
                }
                (None, Some(album)) => {
                    output.push_str(&format!("{album}\n"));
                }
                (None, None) => {}
            }

            // Duration
            if let Some(duration) = track.formatted_duration() {
                output.push_str(&format!("{duration}\n"));
            }

            output.push('\n');
            output.push_str(&format!("▶ {:?} • {}%", status.state, status.volume));
        }

        None => {
            output.push_str("Nothing is currently playing.");
        }
    }

    Ok(output)
}
