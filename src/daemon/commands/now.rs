use crate::player::Player;

pub fn now(player: &Player) -> Result<String, String> {
    let status = player.status();

    let mut output = String::new();

    match status.current_track {
        Some(track) => {
            output.push_str("Now Playing\n");
            output.push_str("-----------\n\n");

            output.push_str(&format!("{}\n", track.display_name()));
            output.push_str(&format!("State  : {:?}\n", status.state));
            output.push_str(&format!("Volume : {}%", status.volume));
        }

        None => {
            output.push_str("Nothing is currently playing.\n");
            output.push_str(&format!("State : {:?}", status.state));
        }
    }

    Ok(output)
}