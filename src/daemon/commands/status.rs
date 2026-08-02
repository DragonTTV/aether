use std::time::Duration;

use crate::player::Player;

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

pub fn status(player: &Player) -> Result<String, String> {
    let status = player.status();

    let mut output = String::new();

    output.push_str("Status\n");
    output.push_str("======\n\n");

    output.push_str(&format!("State    : {:?}\n", status.state));
    output.push_str(&format!("Volume   : {}%\n", status.volume));
    output.push_str(&format!("Repeat   : {}\n", status.repeat));
    output.push_str(&format!(
        "Shuffle  : {}\n\n",
        if status.shuffle { "On" } else { "Off" }
    ));

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

            if let Some(duration) = status.duration {
                output.push_str(&format!(
                    "{} / {}\n",
                    format_duration(status.position),
                    format_duration(duration),
                ));
            } else {
                output.push_str(&format!("{}\n", format_duration(status.position),));
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
