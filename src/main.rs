use clap::Parser;
use aether::cli::{Cli, Command, QueueCommand};

use std::io::{BufReader, Write, Read};
use std::os::unix::net::UnixStream;

fn main() {
    let cli = Cli::parse();
    fn send_command(command: String) {
        let mut stream = UnixStream::connect("/tmp/aether.sock").unwrap();
        writeln!(stream, "{command}").unwrap();
        stream.flush().unwrap();

        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_to_string(&mut response).unwrap();
        println!("{}", response.trim());
    }
    match cli.command {
        Command::Play{source, now} => {
            if now{
                send_command(format!("play_now {}\n", source));
            }else{
                send_command(format!("play {}\n", source));
            }
        }
        Command::Pause=>{
            send_command("pause\n".to_string());
        }
        Command::Resume => {
            send_command("resume\n".to_string());
        }
        Command::Stop => {
            send_command("stop\n".to_string());
        }
        Command::Volume { level} => {
            send_command(format!("volume {}\n", level));
        }
        Command::Status => {
            send_command("status".to_string());        
        }
        Command::Now => {
            send_command("now".into());
        }
        Command::Next => {
            send_command("next \n".to_string());
        }
        Command::Prev => {
            send_command("prev \n".to_string());
        }
        Command::Queue {subcommand}=> {
            match subcommand{
                QueueCommand::Add {source}=> {
                    send_command(format!("queue add {}", source));
                }
                QueueCommand::Remove {index}=> {
                    send_command(format!("queue remove {}", index));
                }
                QueueCommand::Clear => {
                    send_command("queue clear".to_string());
                }
                QueueCommand::List => {
                    send_command("queue list".to_string());
                }
            }
        }
        _ => {
            println!("Not implemented yet");
        }
    }
}