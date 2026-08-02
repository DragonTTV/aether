use crate::platform::daemon::DaemonStatus;

pub fn ensure_running() -> Result<(), String> {
    Err("Windows not implemented yet".into())
}

pub fn start() -> Result<(), String> {
    Err("Windows not implemented yet".into())
}

pub fn stop() -> Result<(), String> {
    Err("Windows not implemented yet".into())
}

pub fn restart() -> Result<(), String> {
    Err("Windows not implemented yet".into())
}



pub fn status() -> Result<DaemonStatus, String> {
    Err("Windows not implemented".into())
}