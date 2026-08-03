#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
mod windows;

pub mod common;
pub mod ui;

pub fn install() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        linux::install::install()
    }

    #[cfg(target_os = "windows")]
    {
        windows::install()
    }
}

pub fn uninstall() -> Result<(), String> {
    Ok(())
}

pub fn doctor() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        linux::doctor::doctor()
    }

    #[cfg(target_os = "windows")]
    {
        //TODO
    }
}

pub fn update() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        linux::update::update()
    }

    #[cfg(target_os = "windows")]
    {
        //TODO
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallState {
    Install,
    Reinstall,
}
