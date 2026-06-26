use clap::Parser;
use aether::cli::{Cli, Command};

use std::io::Write;
use std::os::unix::net::UnixStream;

fn main() {
    let cli = Cli::parse();
    fn send_command(command: String) {
        let mut stream = UnixStream::connect("/tmp/aether.sock").unwrap();
        stream.write_all(command.as_bytes()).unwrap();
    }
    match cli.command {
        Command::Play{source} => {
            send_command(format!("play {}\n", source));
            println!("Sent play command!");
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
        _ => {
            println!("Not implemented yet");
        }
    }
}