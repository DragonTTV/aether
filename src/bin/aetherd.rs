use std::fs;
use std::os::unix::net::UnixListener;
use std::time::Duration;

use aether::daemon::{handler, pid, constants::SOCKET_PATH, lifecycle};
use aether::player::Player;

fn main() {
    let mut player = Player::new();

    let socket_path = SOCKET_PATH;

    // Prevent multiple daemon instances.
    if pid::daemon_running() {
        eprintln!("Aether daemon is already running.");
        return;
    }

    // Write our PID.
    pid::write_pid().expect("Failed to write PID file");

    // Remove any stale socket left behind.
    let _ = fs::remove_file(socket_path);

    let listener = UnixListener::bind(socket_path).unwrap();
    listener.set_nonblocking(true).unwrap();

    println!("Aether daemon listening on {}", socket_path);
    let mut shutdown = false;
    while !shutdown {
        let handled_command = match listener.accept() {
            Ok((stream, _)) => {
                handler::handle(stream, &mut player, &mut shutdown);
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
    lifecycle::cleanup();
}