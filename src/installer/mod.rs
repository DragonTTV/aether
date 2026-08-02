#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

pub mod ui;

pub fn install() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        return linux::install();
    }

    #[cfg(target_os = "windows")]
    {
        return windows::install();
    }
}

pub fn uninstall() -> Result<(), String>{
    Ok(())
}

pub fn doctor() -> Result<(), String>{
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallState {
    Install,
    Reinstall,
}

