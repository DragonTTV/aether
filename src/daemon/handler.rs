use std::io::{BufRead, BufReader, Write};
use std::os::linux::raw::stat;
use std::os::unix::net::UnixStream;

use crate::daemon::commands::{playback, queue, status};
use crate::player::Player;

pub fn handle(mut stream: UnixStream, player: &mut Player) {
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
            }else{
                let queue_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();
                queue::handle(
                    queue_parts[0],
                    queue_parts.get(1).copied(),
                    player,
                )
            }
        }
        "status" => status::status(player),
        _ => {
            playback::handle(
                parts[0],
                parts.get(1).copied(),
                player,
            )
        }
    };
    match response {
        Ok(messsage)=>{
            writeln!(stream, "{messsage}").unwrap()
        }
        Err(message)=>{
            writeln!(stream, "Error: {message}").unwrap()
        }
    }
}