use std::fs;
use std::os::unix::net::UnixListener;
use std::sync::{Arc, Mutex};
use std::time::Duration;


use aether::daemon::{constants::SOCKET_PATH, handler, lifecycle, pid};
use aether::library::{Library, storage};
use aether::mpris::server::MprisServer;
use aether::player::Player;
use aether::database::Database;
use aether::player::state::UpdateEvent;

#[tokio::main]
async fn main() {
    let player = Arc::new(Mutex::new(Player::new()));
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
   
    let mpris = MprisServer::new(player.clone())
        .await
        .expect("Failed to start MPRIS");

    while !shutdown {
        let handled_command = match listener.accept() {
            Ok((stream, _)) => {
                handler::handle(stream, &player, &mut library, &mut database,&mut shutdown, &mpris).await;
                true
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => false,

            Err(e) => {
                eprintln!("{e}");
                false
            }
        };

        match player.lock().unwrap().update(){
            UpdateEvent::TrackChanged => {
                // let _ = mpris.notify_metadata().await;
                // let _ = mpris.notify_playback_status().await;
                // let _ = mpris.notify_position().await;
            }
            UpdateEvent::None => {}
            UpdateEvent::PlaybackStopped => {}
        };

        if !handled_command {
            std::thread::sleep(Duration::from_millis(50));
        }
    }
    lifecycle::cleanup();
}
