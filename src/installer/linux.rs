use std::env;
use std::path::Path;
use std::process::Command;
use std::{fs, path::PathBuf};

use crate::{
    installer::{InstallState, ui},
    platform,
};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub fn install() -> Result<(), String> {
    ui::header();
    ui::step("Installing Aether...");
    check_environment()?;
    let state = detect_installation()?;
    confirm_installation(state)?;
    create_directories()?;
    install_binaries()?;
    configure_path()?;
    install_service()?;
    enable_service()?;
    finish();
    Ok(())
}

fn check_environment() -> Result<(), String> {
    ui::step("Checking environment...");

    check_systemd()?;
    check_user_systemd()?;

    ui::success("Environment OK");

    Ok(())
}

fn check_systemd() -> Result<(), String> {
    let output = Command::new("systemctl")
        .arg("--version")
        .output()
        .map_err(|_| {
            "Aether currently requires systemd.\n\
             Support for non-systemd init systems may be added in the future."
                .to_string()
        })?;

    if !output.status.success() {
        return Err("Aether currently requires systemd.\n\
             Support for non-systemd init systems may be added in the future."
            .into());
    }

    ui::success("systemd detected");

    Ok(())
}

fn check_user_systemd() -> Result<(), String> {
    let output = Command::new("systemctl")
        .args(["--user", "show-environment"])
        .output()
        .map_err(|_| "Unable to communicate with the systemd user manager.".to_string())?;

    if !output.status.success() {
        return Err("Unable to communicate with the systemd user manager.".into());
    }

    ui::success("systemd user services available");

    Ok(())
}

fn detect_installation() -> Result<InstallState, String> {
    let aether = platform::bin_dir().join("aether");
    let daemon = platform::bin_dir().join("aetherd");
    let service = platform::service_dir().join("aetherd.service");

    if aether.exists() && daemon.exists() && service.exists() {
        Ok(InstallState::Reinstall)
    } else {
        Ok(InstallState::Install)
    }
}

fn confirm_installation(state: InstallState) -> Result<(), String> {
    ui::newline();

    match state {
        InstallState::Install => {
            ui::info("Installation Type : Install");
        }
        InstallState::Reinstall => {
            ui::warning("Existing Aether installation detected.");
            ui::info("Installation Type : Reinstall");
        }
    }

    ui::info(&format!(
        "Target            : {}",
        platform::data_dir().display()
    ));

    ui::newline();

    ui::info("The following will be installed:");
    ui::info(" • Aether CLI");
    ui::info(" • Aether Daemon");
    ui::info(" • systemd user service");
    ui::newline();

    if !ui::confirm("Continue?")? {
        return Err("Installation cancelled.".into());
    }
    Ok(())
}

fn create_directories() -> Result<(), String> {
    ui::step("Creating directories...");

    create_directory(&platform::bin_dir())?;
    create_directory(&platform::data_dir())?;
    create_directory(&platform::artwork_dir())?;
    create_directory(&platform::service_dir())?;

    Ok(())
}

const BINARIES: &[&str] = &["aether", "aetherd", "aether-setup"];

fn install_binaries() -> Result<(), String> {
    ui::step("Installing binaries...");

    let installer_dir = installer_directory()?;

    for binary in BINARIES {
        install_binary(&installer_dir, binary)?;
    }

    Ok(())
}

fn installer_directory() -> Result<PathBuf, String> {
    let exe =
        env::current_exe().map_err(|e| format!("Failed to determine installer location: {e}"))?;

    exe.parent()
        .map(PathBuf::from)
        .ok_or_else(|| "Failed to determine installer directory.".to_string())
}

fn install_binary(installer_dir: &Path, binary: &str) -> Result<(), String> {
    let source = installer_dir.join(binary);
    let destination = platform::bin_dir().join(binary);

    if !source.exists() {
        return Err(format!("Missing binary: {}", source.display()));
    }

    install_file(&source, &destination)?;
    #[cfg(unix)]
    set_executable(&destination)?;

    ui::success(&format!("Installed {binary}"));

    Ok(())
}

#[cfg(unix)]
fn set_executable(path: &Path) -> Result<(), String> {
    let mut permissions = fs::metadata(path).map_err(|e| e.to_string())?.permissions();

    permissions.set_mode(0o755);

    fs::set_permissions(path, permissions).map_err(|e| e.to_string())?;

    Ok(())
}

fn install_service() -> Result<(), String> {
    ui::step("Installing systemd service...");

    let installer_dir = installer_directory()?;

    let source = installer_dir
        .join("assets")
        .join("linux")
        .join("aetherd.service");

    let destination = platform::service_dir().join("aetherd.service");

    install_file(&source, &destination)?;
    ui::success("Installed aetherd.service");
    Ok(())
}

fn install_file(source: &Path, destination: &Path) -> Result<(), String> {
    if !source.exists() {
        return Err(format!("Missing file: {}", source.display()));
    }

    fs::copy(source, destination).map_err(|e| {
        format!(
            "Failed to copy '{}' to '{}': {}",
            source.display(),
            destination.display(),
            e
        )
    })?;

    Ok(())
}

fn enable_service() -> Result<(), String> {
    ui::step("Configuring systemd service...");

    // Reload user services
    let status = Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .status()
        .map_err(|e| format!("Failed to reload systemd daemon: {e}"))?;

    if !status.success() {
        return Err("Failed to reload systemd daemon.".into());
    }

    ui::success("Reloaded systemd daemon");

    // Enable service
    let status = Command::new("systemctl")
        .args(["--user", "enable", "aetherd.service"])
        .status()
        .map_err(|e| format!("Failed to enable aetherd.service: {e}"))?;

    if !status.success() {
        return Err("Failed to enable aetherd.service.".into());
    }

    ui::success("Enabled aetherd.service");

    Ok(())
}

fn create_directory(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())?;

    ui::success(&format!("{}", path.display()));

    Ok(())
}

fn configure_path() -> Result<(), String> {
    use std::fs::{self, OpenOptions};
    use std::io::Write;

    ui::step("Checking PATH...");

    let bin_dir = platform::bin_dir();

    let path = env::var("PATH").unwrap_or_default();

    if path
        .split(':')
        .any(|p| Path::new(p) == bin_dir.as_path())
    {
        ui::success("PATH already configured");
        return Ok(());
    }

    ui::warning(&format!(
        "{} is not on your PATH.",
        bin_dir.display()
    ));

    if !ui::confirm("Would you like Aether to configure it automatically?")? {
        ui::warning("Skipping PATH configuration.");
        return Ok(());
    }

    let shell = env::var("SHELL").unwrap_or_default();

    let (config, line, reload) = if shell.ends_with("fish") {
        (
            dirs::home_dir()
                .unwrap()
                .join(".config/fish/config.fish"),
            "fish_add_path ~/.local/bin",
            "exec fish",
        )
    } else if shell.ends_with("bash") {
        (
            dirs::home_dir().unwrap().join(".bashrc"),
            "export PATH=\"$HOME/.local/bin:$PATH\"",
            "source ~/.bashrc",
        )
    } else if shell.ends_with("zsh") {
        (
            dirs::home_dir().unwrap().join(".zshrc"),
            "export PATH=\"$HOME/.local/bin:$PATH\"",
            "source ~/.zshrc",
        )
    } else {
        ui::warning("Unsupported shell.");
        ui::info("Please add the following directory to your PATH manually:");
        ui::info(&format!("  {}", bin_dir.display()));
        return Ok(());
    };

    if let Some(parent) = config.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let existing = fs::read_to_string(&config).unwrap_or_default();

    if !existing.contains(line) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&config)
            .map_err(|e| e.to_string())?;

        writeln!(file).map_err(|e| e.to_string())?;
        writeln!(file, "{line}").map_err(|e| e.to_string())?;
    }

    ui::success("PATH configured.");
    ui::newline();
    ui::info("Restart your terminal or run:");
    ui::info(&format!("  {reload}"));

    Ok(())
}

fn finish() {
    ui::newline();

    ui::success("Installation completed successfully!");
    ui::newline();

    ui::info("Run:");
    ui::info("  aether");
    ui::newline();

    ui::info("The daemon will start automatically when needed.");
    ui::newline();
    ui::info("If PATH was updated, restart your terminal before running Aether.");
}
