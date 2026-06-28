use std::os::unix::net::UnixListener;
use std::fs;
use std::io::{BufRead, BufReader};
use aether::player::{Player, Track};
fn main() {
    let mut player = Player::new();
    let socket_path = "/tmp/aether.sock";

    let _ = fs::remove_file(socket_path);
    
    let listener = UnixListener::bind(socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();
    println!("Aether daemon listening on {}", socket_path);

    loop{
        let handled_command = match listener.accept() {
            Ok((stream, _)) => {
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
                    "play_now" => {
                        todo!();
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
                        player.set_volume(level);
                    }
                    "next" => {
                        player.next();
                    }
                    "previous" => {
                        player.previous();
                    }
                    "queue" => {
                        if parts.len() < 2 {
                            continue;
                        }
                        let queue_parts: Vec<&str> = parts[1].splitn(2, ' ').collect();
                        match queue_parts[0]{
                            "list" => {
                                for (i, track) in player.queue().tracks().iter().enumerate() {
                                    println!("{i}: {}", track.source);
                                }
                            }
                            "clear" => {
                                player.queue.clear();
                            }
                            _ => {}
                        }
                    }
                    _ => {eprintln!("Unknown command: {}", parts[0]);}
                }
                true
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {false}
            Err(e) => {
                eprintln!("{e}");
                false
            },
        };
        player.update();
        if !handled_command {std::thread::sleep(std::time::Duration::from_millis(50));}
    }
}