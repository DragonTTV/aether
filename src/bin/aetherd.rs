use std::os::unix::net::UnixListener;
use std::fs;
use std::io::{BufRead, BufReader};
use aether::player::{Player, Track};
fn main() {
    let mut player = Player::new();
    let socket_path = "/tmp/aether.sock";

    let _ = fs::remove_file(socket_path);
    
    let listener = UnixListener::bind(socket_path).unwrap();

    println!("Aether daemon listening on {}", socket_path);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut reader = BufReader::new(stream);

                let mut command = String::new();
                reader.read_line(&mut command).unwrap();

                let parts: Vec<&str> = command.trim().splitn(2, ' ').collect();

                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    "play" => {
                        if parts.len() < 2 {
                            continue;
                        }
                        let track = Track::new(parts[1].to_string());
                        player.play(track);
                    }
                    "pause" => {
                        player.pause();
                    }
                    "resume" => {
                        player.resume();
                    }
                    "stop" =>{
                        player.stop();
                    }
                    "volume" => {
                        if parts.len() <2 {
                            continue;
                        }
                        let level = match parts[1].parse::<u8>() {
                            Ok(level) => level,
                            Err(_) => continue,
                        };
                        println!("Volume set to: {}", level);
                        player.set_volume(level);
                    }
                    _ => {eprintln!("Unknown command: {}", parts[0]);}
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}