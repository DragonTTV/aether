use std::fs;
use std::os::unix::net::UnixListener;
use std::time::Duration;


use aether::daemon::{constants::SOCKET_PATH, handler, lifecycle, pid};
use aether::library::{Library, storage};
use aether::player::Player;
use aether::database::Database;

fn main() {
    let mut player = Player::new();
    // let mut library = storage::load().unwrap_or_else(|_| Library::new());

    let mut database = Database::open().expect("Failed to open Aether database");
    database.initialize().expect("Failed to initialize Aether database");
    let mut library = if database.is_library_empty().expect("Failed to check database state"){
        let json_library = storage::load().unwrap_or_else(|_| Library::new());
        if !json_library.is_empty() || !json_library.scan_paths().is_empty(){
            println!("Migrating library data to SQLite...");
            database.save_library(&json_library).expect("Failed to migrate library to database");
            storage::mark_as_migrated().expect("Failed to mark old library data as migrated");
            println!("Library migration complete.");
        }
        json_library
    }else{
        database
        .load_library()
        .expect("Failed to load library from database")
    };
    
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
                handler::handle(stream, &mut player, &mut library, &mut database,&mut shutdown);
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
