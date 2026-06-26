use clap::Parser;
use aether::cli::{Cli, Command};

use std::io::Write;
use std::os::unix::net::UnixStream;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Play{source} => {
            let mut stream = UnixStream::connect("/tmp/aether.sock").unwrap();

            let command = format!("play {}\n", source);
            stream.write_all(command.as_bytes()).unwrap();

            println!("Sent play command!");
        }

        _ => {
            println!("Not implemented yet");
        }
    }
}