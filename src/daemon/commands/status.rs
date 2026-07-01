use crate::player::Player;

pub fn status(player: &Player) -> Result<String, String> {
    let status = player.status();

    let mut output = String::new();

    output.push_str(&format!("State  : {:?}\n", status.state));
    output.push_str(&format!("Volume : {}%\n", status.volume));

    match &status.current_track {
        Some(track) => {
            output.push_str(&format!(
                "Now Playing : {}\n",
                track.source
            ));
        }
        None => output.push_str("Now Playing : Nothing\n"),
    }

    output.push_str("\nQueue:\n");

    for (i, track) in status.queue.iter().enumerate() {
        if Some(i) == status.current_index {
            output.push_str(&format!("▶ {}\n", track.source));
        } else {
            output.push_str(&format!("  {}\n", track.source));
        }
    }

    Ok(output)
}