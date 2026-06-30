use std::fs;
use std::os::unix::net::UnixListener;
use std::time::Duration;

use aether::daemon::handler;
use aether::player::Player;

fn main() {
    let mut player = Player::new();
    let socket_path = "/tmp/aether.sock";

    let _ = fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();

    println!("Aether daemon listening on {}", socket_path);

    loop {
        let handled_command = match listener.accept() {
            Ok((stream, _)) => {
                handler::handle(stream, &mut player);
                true
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => false,

            Err(e) => {
                eprintln!("{e}");
                false
            }
        };

        player.update();

        if !handled_command {
            std::thread::sleep(Duration::from_millis(50));
        }
    }
}