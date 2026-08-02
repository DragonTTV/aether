use std::fs;
use std::os::unix::net::UnixListener;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use aether::daemon::{handler, lifecycle, pid};
use aether::database::Database;
use aether::ipc::path::socket_path;
use aether::library::{Library, storage};
use aether::mpris::server::MprisServer;
use aether::player::Player;
use aether::player::state::UpdateEvent;

#[tokio::main]
async fn main() {
    let player = Arc::new(Mutex::new(Player::new()));

    let mut database = Database::open().expect("Failed to open Aether database");
    database
        .initialize()
        .expect("Failed to initialize Aether database");
    let mut library = if database
        .is_library_empty()
        .expect("Failed to check database state")
    {
        let json_library = storage::load().unwrap_or_else(|_| Library::new());
        if !json_library.is_empty() || !json_library.scan_paths().is_empty() {
            println!("Migrating library data to SQLite...");
            database
                .save_library(&json_library)
                .expect("Failed to migrate library to database");
            storage::mark_as_migrated().expect("Failed to mark old library data as migrated");
            println!("Library migration complete.");
        }
        json_library
    } else {
        database
            .load_library()
            .expect("Failed to load library from database")
    };

    if pid::daemon_running() {
        eprintln!("Aether daemon is already running.");
        return;
    }

    pid::write_pid().expect("Failed to write PID file");

    let _ = fs::remove_file(socket_path());

    let listener = UnixListener::bind(socket_path()).unwrap();
    listener.set_nonblocking(true).unwrap();

    println!("Aether daemon listening on {}", socket_path().display());
    let mut shutdown = false;

    let shutdown_requested = Arc::new(AtomicBool::new(false));

    {
        let shutdown_requested = shutdown_requested.clone();

        ctrlc::set_handler(move || {
            shutdown_requested.store(true, Ordering::SeqCst);
        })
        .expect("Failed to install signal handler");
    }

    let mpris = MprisServer::new(player.clone())
        .await
        .expect("Failed to start MPRIS");

    while !shutdown && !shutdown_requested.load(Ordering::SeqCst) {
        let handled_command = match listener.accept() {
            Ok((stream, _)) => {
                handler::handle(
                    stream,
                    &player,
                    &mut library,
                    &mut database,
                    &mut shutdown,
                    &mpris,
                )
                .await;
                true
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => false,

            Err(e) => {
                eprintln!("{e}");
                false
            }
        };
        let event = {
            let mut player = player.lock().unwrap();
            player.update()
        };
        match event {
            UpdateEvent::TrackChanged => {
                let _ = mpris.notify_metadata().await;
                let _ = mpris.notify_can_play_pause().await;
                let _ = mpris.notify_playback_status().await;
                let _ = mpris.notify_position().await;
            }
            UpdateEvent::RepeatChanged => {
                let _ = mpris.notify_loop_status().await;
            }
            UpdateEvent::ShuffleChanged => {
                let _ = mpris.notify_shuffle_status().await;
            }
            UpdateEvent::VolumeChanged => {
                let _ = mpris.notify_volume_status().await;
            }
            UpdateEvent::PlaybackStopped => {
                let _ = mpris.notify_playback_status().await;
            }
            UpdateEvent::None => {}
        };

        if !handled_command {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }
    lifecycle::cleanup();
}
