use crate::player::Player;

pub fn status(player: &Player) -> Result<String, String> {
    let status = player.status();

    let mut output = String::new();

    output.push_str("Status\n");
    output.push_str("======\n\n");

    output.push_str(&format!("State    : {:?}\n", status.state));
    output.push_str(&format!("Volume   : {}%\n\n", status.volume));

    match &status.current_track {
        Some(track) => {
            output.push_str("Now Playing\n");
            output.push_str("-----------\n\n");

            output.push_str(track.display_name());
            output.push('\n');

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

            if let Some(duration) = track.formatted_duration() {
                output.push_str(&format!("{duration}\n"));
            }

            output.push('\n');
        }

        None => {
            output.push_str("Nothing is currently playing.\n\n");
        }
    }

    output.push_str("Queue\n");
    output.push_str("-----\n\n");

    if status.queue.is_empty() {
        output.push_str("(empty)");
    } else {
        for (i, track) in status.queue.iter().enumerate() {
            let marker = if Some(i) == status.current_index {
                "▶"
            } else {
                " "
            };

            output.push_str(&format!("{marker} {}\n", track.display_name()));
        }
    }

    Ok(output)
}