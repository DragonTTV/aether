use crate::daemon::constants::SOCKET_PATH;
use crate::daemon::pid;
use std::{
    fs, io,
    path::Path,
    process::{Command, Stdio},
    thread,
    time::Duration,
};
pub fn cleanup() {
    let _ = pid::remove_pid();
    let _ = fs::remove_file(SOCKET_PATH);
}

pub fn start_daemon() -> io::Result<()> {
    #[cfg(debug_assertions)]
    {
        Command::new("cargo")
            .args(["run", "--bin", "aetherd"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

    #[cfg(not(debug_assertions))]
    {
        Command::new("aetherd")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
    }

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
        if Path::new(SOCKET_PATH).exists() {
            "Connected"
        } else {
            "Missing"
        }
    );
}

fn wait_for_socket() -> io::Result<()> {
    for _ in 0..50 {
        if Path::new(SOCKET_PATH).exists() {
            return Ok(());
        }

        thread::sleep(Duration::from_millis(20));
    }

    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        "Timed out waiting for daemon to start.",
    ))
}
