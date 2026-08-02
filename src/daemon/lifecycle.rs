use crate::{platform};
use crate::ipc::path::socket_path;
use crate::daemon::pid;
use std::{
    fs, io,
    path::Path,
    thread,
    time::Duration,
};
pub fn cleanup() {
    let _ = pid::remove_pid();
    let _ = fs::remove_file(socket_path());
}

pub fn start_daemon() -> io::Result<()> {
    platform::ensure_daemon_running()
        .map_err(io::Error::other)?;

    wait_for_socket()?;

    Ok(())
}
pub fn daemon_restart() {}
pub fn daemon_status() {
    println!("Daemon");
    println!("======\n");

    if !pid::daemon_running() {
        println!("Status : Stopped");
        return;
    }

    let pid = pid::read_pid().unwrap_or(0);

    println!("Status : Running");
    println!("PID    : {}", pid);

    println!(
        "Socket : {}",
        if Path::new(&socket_path()).exists() {
            "Connected"
        } else {
            "Missing"
        }
    );
}

fn wait_for_socket() -> io::Result<()> {
    for _ in 0..50 {
        if Path::new(&socket_path()).exists() {
            return Ok(());
        }

        thread::sleep(Duration::from_millis(20));
    }

    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "Timed out waiting for daemon to start.",
    ))
}
