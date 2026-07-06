use aether::daemon::{lifecycle, pid};
// use aether::library::{Library};
use clap::Parser;
use aether::cli::{Cli, Command, QueueCommand, DaemonCommand, LibraryCommand};
use aether::daemon::{constants::SOCKET_PATH};
use std::io::{BufReader, Write, Read};
use std::os::unix::net::UnixStream;
// use std::path::Path;
// use aether::library::{Library, scanner};

fn send_command(command: String) -> Result<(), String> {
    let mut stream = match UnixStream::connect(SOCKET_PATH) {
        Ok(stream) => stream,

        Err(_) => {
            lifecycle::start_daemon()
                .map_err(|e| format!("Failed to start daemon: {e}"))?;

            UnixStream::connect(SOCKET_PATH)
                .map_err(|e| format!("Failed to connect to daemon after startup: {e}"))?
        }
    };

    writeln!(stream, "{command}")
        .map_err(|e| e.to_string())?;

    stream.flush()
        .map_err(|e| e.to_string())?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();

    reader
        .read_to_string(&mut response)
        .map_err(|e| e.to_string())?;

    println!("{}", response.trim());

    Ok(())
}


fn main() {
    let cli = Cli::parse();
    // let mut library = Library::new();
    
    // scanner::scan(
    //     Path::new("/home/dragon/Music"),
    //     &mut library,
    // ).unwrap();
    
    // println!("Found {} tracks", library.len());

    // for track in library.tracks() {
    //     println!("{}", track.display_name());
    // }

    match cli.command {
        Command::Play{source, now, id} => {
            let command = match(now, id){
                (false, false) => format!("play {}", source),
                (true, false) => format!("play_now {}", source),
                (false, true) => format!("play_id {}", source),
                (true, true) => format!("play_now_id {}", source),
            };
            send_command(command).unwrap();
        }
        Command::Pause=>{
            send_command("pause".to_string()).unwrap();
        }
        Command::Resume => {
            send_command("resume".to_string()).unwrap();
        }
        Command::Stop => {
            send_command("stop".to_string()).unwrap();
        }
        Command::Volume { level} => {
            send_command(format!("volume {}\n", level)).unwrap();
        }
        Command::Status => {
            send_command("status".to_string()).unwrap();        
        }
        Command::Now => {
            send_command("now".into()).unwrap();
        }
        Command::Next => {
            send_command("next \n".to_string()).unwrap();
        }
        Command::Prev => {
            send_command("prev \n".to_string()).unwrap();
        }
        Command::Queue {subcommand}=> {
            match subcommand{
                QueueCommand::Add {source, id}=> {
                    if id {
                        send_command(format!("queue add_id {}", source)).unwrap();
                    } else {
                        send_command(format!("queue add {}", source)).unwrap();
                    }
                }
                QueueCommand::Remove {index}=> {
                    send_command(format!("queue remove {}", index)).unwrap();
                }
                QueueCommand::Clear => {
                    send_command("queue clear".to_string()).unwrap();
                }
                QueueCommand::List => {
                    send_command("queue list".to_string()).unwrap();
                }
            }
        }
        Command::Daemon { subcommand } => match subcommand {
            DaemonCommand::Status => {
                lifecycle::daemon_status();
            }
            DaemonCommand::Start => {
                if pid::daemon_running(){
                    println!("Daemon is already is running.");
                }else{
                    lifecycle::start_daemon().expect("Failed to start daemon.");
                    println!("Daemon Started.")
                }
            }
            DaemonCommand::Stop => {
                send_command("daemon stop".to_string()).unwrap();
            }
            DaemonCommand::Restart => {
                send_command("daemon stop".to_string()).unwrap();

                while pid::daemon_running() {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }

                lifecycle::start_daemon().expect("Failed to start daemon.");

                println!("Daemon restarted.");
            }
        }
        Command::Library { subcommand } => match subcommand {
            LibraryCommand::Scan { path } => {
                send_command(format!("library scan {}", path)).unwrap();
            }

            LibraryCommand::List {sort} => {
                if let Some(sort) = sort {
                    send_command(format!("library list {}", sort.as_str())).unwrap();
                } else {
                    send_command("library list".into()).unwrap();
                }
            }
            LibraryCommand::Search { query } => {
                send_command(format!("library search {}", query)).unwrap();
            }
            LibraryCommand::Info {id}=> {
                send_command(format!("library info {}", id)).unwrap();
            }
            LibraryCommand::Rescan => {
                send_command("library rescan".into()).unwrap();
            }
        }   
        _ => {
            println!("Not implemented yet");
        }
    }
}