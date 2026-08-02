use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

pub mod daemon;
use daemon::DaemonStatus;

pub fn ensure_daemon_running() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        return linux::daemon::ensure_running();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::daemon::ensure_running();
    }
}

pub fn start_daemon() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        return linux::daemon::start();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::daemon::start();
    }
}

pub fn stop_daemon() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        return linux::daemon::stop();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::daemon::stop();
    }
}

pub fn restart_daemon() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        return linux::daemon::restart();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::daemon::restart();
    }
}

pub fn daemon_status() -> Result<DaemonStatus, String> {
    #[cfg(target_os = "linux")]
    {
        return linux::daemon::status();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::daemon::status();
    }
}



pub fn bin_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        return linux::bin_dir();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::bin_dir();
    }
}

pub fn data_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        return linux::data_dir();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::data_dir();
    }
}

pub fn artwork_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        return linux::artwork_dir();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::artwork_dir();
    }
}

pub fn service_dir() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        return linux::service_dir();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::service_dir();
    }
}