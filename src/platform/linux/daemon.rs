use std::process::Command;

use crate::platform::daemon::DaemonStatus;

pub fn is_running() -> bool {
    Command::new("systemctl")
        .arg("--user")
        .args(["is-active", "--quiet", "aetherd.service"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn start() -> Result<(), String> {
    let status = Command::new("systemctl")
        .arg("--user")
        .args(["start", "aetherd.service"])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to start aetherd.service".into())
    }
}

pub fn stop() -> Result<(), String> {
    let status = Command::new("systemctl")
        .arg("--user")
        .args(["stop", "aetherd.service"])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to stop aetherd.service".into())
    }
}

pub fn restart() -> Result<(), String> {
    let status = Command::new("systemctl")
        .arg("--user")
        .args(["restart", "aetherd.service"])
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to restart aetherd.service".into())
    }
}

pub fn status() -> Result<DaemonStatus, String> {
    if is_running() {
        Ok(DaemonStatus::Running)
    } else {
        Ok(DaemonStatus::Stopped)
    }
}

pub fn ensure_running() -> Result<(), String> {
    if is_running() {
        return Ok(());
    }

    start()
}
