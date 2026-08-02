use aether::daemon::{lifecycle};
use aether::cli::{Cli, Command, DaemonCommand, LibraryCommand, PlaylistCommand, QueueCommand, RepeatModeArg, ShuffleModeArg};
use aether::ipc::path::socket_path;
use aether::platform;
use aether::platform::daemon::DaemonStatus;
use clap::Parser;
use std::io::{BufReader, Read, Write};
use std::os::unix::net::UnixStream;

fn send_command(command: String) -> Result<(), String> {
    let mut stream = match UnixStream::connect(socket_path()) {
        Ok(stream) => stream,

        Err(_) => {
            lifecycle::start_daemon().map_err(|e| format!("Failed to start daemon: {e}"))?;

            UnixStream::connect(socket_path())
                .map_err(|e| format!("Failed to connect to daemon after startup: {e}"))?
        }
    };

    writeln!(stream, "{command}").map_err(|e| e.to_string())?;

    stream.flush().map_err(|e| e.to_string())?;

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

    match cli.command {
        Command::Play { source, now, id,playlist } => {
            let command = match (now, id, playlist) {
                (false, false, false) => format!("play {}", source),
                (true, false, false) => format!("play_now {}", source),
                (false, true, false) => format!("play_id {}", source),
                (true, true, false) => format!("play_now_id {}", source),
                (false, false, true) => format!("play_playlist {source}"),
                (true, false, true) => format!("play_now_playlist {source}"),
                _ => unreachable!("Clap prevents conflicting play source types")
            };
            send_command(command).unwrap();
        }
        Command::Pause => {
            send_command("pause".to_string()).unwrap();
        }
        Command::Resume => {
            send_command("resume".to_string()).unwrap();
        }
        Command::Stop => {
            send_command("stop".to_string()).unwrap();
        }
        Command::Volume { level } => {
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
        Command::Queue { subcommand } => match subcommand {
            QueueCommand::Add { source, id } => {
                if id {
                    send_command(format!("queue add_id {}", source)).unwrap();
                } else {
                    send_command(format!("queue add {}", source)).unwrap();
                }
            }
            QueueCommand::Remove { index } => {
                send_command(format!("queue remove {}", index)).unwrap();
            }
            QueueCommand::Clear => {
                send_command("queue clear".to_string()).unwrap();
            }
            QueueCommand::List => {
                send_command("queue list".to_string()).unwrap();
            }
        },
       Command::Daemon { subcommand } => match subcommand {
            DaemonCommand::Status => {
                match platform::daemon_status().expect("Failed to query daemon status.") {
                    DaemonStatus::Running => println!("Daemon is running."),
                    DaemonStatus::Stopped => println!("Daemon is stopped."),
                }
            }

            DaemonCommand::Start => {
                platform::start_daemon().expect("Failed to start daemon.");
                println!("Daemon started.");
            }

            DaemonCommand::Stop => {
                platform::stop_daemon().expect("Failed to stop daemon.");
                println!("Daemon stopped.");
            }

            DaemonCommand::Restart => {
                platform::restart_daemon().expect("Failed to restart daemon.");
                println!("Daemon restarted.");
            }
        },
        Command::Library { subcommand } => match subcommand {
            LibraryCommand::Scan { path } => {
                send_command(format!("library scan {}", path)).unwrap();
            }

            LibraryCommand::List { sort } => {
                if let Some(sort) = sort {
                    send_command(format!("library list {}", sort.as_str())).unwrap();
                } else {
                    send_command("library list".into()).unwrap();
                }
            }
            LibraryCommand::Search { query } => {
                send_command(format!("library search {}", query)).unwrap();
            }
            LibraryCommand::Info { id } => {
                send_command(format!("library info {}", id)).unwrap();
            }
            LibraryCommand::Rescan {reid}=> {
                if reid{
                    send_command("library rescan_reid".into()).unwrap()
                }
                else{
                    send_command("library rescan".into()).unwrap();
                }
            }
        },
        Command::Playlist { subcommand} => match subcommand {
            PlaylistCommand::Create { name } => {
                send_command(format!("playlist create {}", name)).unwrap();
            }
            PlaylistCommand::List => {
                send_command("playlist list".to_string()).unwrap();
            }
            PlaylistCommand::Show { id } => {
                send_command(format!("playlist show {}", id)).unwrap();
            }
            PlaylistCommand::Add {playlist_id,track_ids} => {
                let ids = track_ids
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                send_command(format!("playlist add {playlist_id} {ids}")).unwrap();
            }
            PlaylistCommand::Remove {playlist_id, position, all, missing} => {
                if all {
                    send_command(format!("playlist remove_all {playlist_id}")).unwrap();
                } else if missing{
                    send_command(format!("playlist remove_missing {playlist_id}")).unwrap();
                } 
                else {
                    send_command(format!("playlist remove {playlist_id} {}", position.unwrap())).unwrap();
                }
            }
            PlaylistCommand::Delete { id } => {
                send_command(format!("playlist delete {id}")).unwrap();
            }
            PlaylistCommand::Rename { id, name } => {
                send_command(format!("playlist rename {id} {name}")).unwrap();
            }
            PlaylistCommand::Move { playlist_id, from, to } => {
                send_command(format!("playlist move {playlist_id} {from} {to}")).unwrap();
            }
            PlaylistCommand::Info { id } => {
                send_command(format!("playlist info {id}")).unwrap();
            }
        },
        Command::Repeat { mode } => {
            match mode {
                None => send_command("repeat".into()).unwrap(),
                Some(RepeatModeArg::Off) => send_command("repeat off".into()).unwrap(),
                Some(RepeatModeArg::Track) => send_command("repeat track".into()).unwrap(),
                Some(RepeatModeArg::Queue) => send_command("repeat queue".into()).unwrap(),
            }
        }
        Command::Shuffle { enabled } => {
            match enabled {
                None => send_command("shuffle".into()).unwrap(),

                Some(ShuffleModeArg::On) => {
                    send_command("shuffle on".into()).unwrap()
                }

                Some(ShuffleModeArg::Off) => {
                    send_command("shuffle off".into()).unwrap()
                }
            }
        }
        Command::Seek { position } => {
            send_command(format!("seek {}", position)).unwrap();
        }
        _ => {
            println!("Not implemented yet");
        }
    }
}
