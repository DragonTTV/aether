use crate::daemon::constants::PID_FILE;

pub fn write_pid() -> std::io::Result<()>{
    let pid = std::process::id();
    
    std::fs::write(PID_FILE, pid.to_string())
}

pub fn read_pid() -> std::io::Result<u32> {
    let pid = std::fs::read_to_string(PID_FILE)?;

    pid.trim()
        .parse()
        .map_err(|_| std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid PID file",
        ))
}

pub fn remove_pid() -> std::io::Result<()>{
    if std::path::Path::new(PID_FILE).exists() {
        std::fs::remove_file(PID_FILE)?;
    }

    Ok(())
}

pub fn daemon_running() -> bool {
    let Ok(pid) = read_pid() else {
        return false;
    };

    let running = std::path::Path::new(&format!("/proc/{pid}")).exists();

    if !running {
        let _ = remove_pid();
    }

    running
}

pub fn current_pid() -> u32 {
    std::process::id()
}