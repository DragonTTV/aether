use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

use crate::daemon::commands::{daemon, library, now, playback, playlist, queue, status};
use crate::library::Library;
use crate::player::Player;
use crate::database::Database;

pub fn handle(
    mut stream: UnixStream,
    player: &mut Player,
    library: &mut Library,
    database: &mut Database,
    shutdown: &mut bool,
) {
    let mut reader = BufReader::new(&mut stream);

    let mut command = String::new();
    reader.read_line(&mut command).unwrap();

    let parts: Vec<&str> = command.trim().splitn(2, ' ').collect();

    if parts.is_empty() {
        return;
    }

    let response = match parts[0] {
        "queue" => {
            if parts.len() < 2 {
                Err("No queue command specified".to_string())
            } else {
                let queue_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();
                queue::handle(queue_parts[0], queue_parts.get(1).copied(), player, library)
            }
        }
        "status" => status::status(player),
        "now" => now::now(player),
        "daemon" => {
            if parts.len() < 2 {
                Err("No daemon command specified".to_string())
            } else {
                let daemon_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();

                daemon::handle(daemon_parts[0], daemon_parts.get(1).copied(), shutdown)
            }
        }
        "library" => {
            if parts.len() < 2 {
                Err("No library command specified".to_string())
            } else {
                let library_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();

                library::handle(library_parts[0], library_parts.get(1).copied(), library, database)
            }
        }
        "playlist" => {
            if parts.len() < 2 {
                Err("No playlist command specified".to_string())
            } else {
                let playlist_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();

                playlist::handle(
                    playlist_parts[0],
                    playlist_parts.get(1).copied(),
                    library,
                    database,
                )
            }
        }
        _ => playback::handle(parts[0], parts.get(1).copied(), player, library, database),
    };
    match response {
        Ok(messsage) => writeln!(stream, "{messsage}").unwrap(),
        Err(message) => writeln!(stream, "Error: {message}").unwrap(),
    }
}
