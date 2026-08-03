use semver::Version;
use std::{fs, os::unix::fs::PermissionsExt as _, path::Path};

use crate::{
    platform,
    setup::{common, ui},
};

pub fn update() -> Result<(), String> {
    ui::header();

    ui::step("Checking installation...");

    let bin_dir = platform::bin_dir();

    if !bin_dir.join("aether").exists()
        || !bin_dir.join("aetherd").exists()
        || !bin_dir.join("aether-setup").exists()
    {
        return Err("Aether is not installed.".into());
    }

    let current = Version::parse(env!("CARGO_PKG_VERSION")).map_err(|e| e.to_string())?;

    ui::step("Checking for updates...");
    let release = common::fetch_latest_release(&current)?;

    let latest =
        Version::parse(release.version.trim_start_matches('v')).map_err(|e| e.to_string())?;

    if latest <= current {
        ui::success("Already up to date.");
        return Ok(());
    }

    ui::step("Downloading...");
    let archive = common::download_release(&release)?;

    if release.checksum_url.is_some() {
        ui::step("Verifying...");
        common::verify_checksum(&archive, &release)?;
    } else {
        ui::warning("Release does not provide a checksum. Skipping verification.");
    }

    ui::step("Stopping daemon...");
    platform::stop_daemon()?;

    ui::step("Extracting...");
    let extracted = common::extract_archive(&archive)?;

    ui::step("Installing the update...");
    install_update(&extracted)?;

    ui::step("Reloading service...");
    platform::daemon_reload()?;

    ui::step("Starting daemon...");
    platform::start_daemon()?;

    ui::success(&format!("Updated to {}", release.version));

    Ok(())
}

fn install_update(extracted: &Path) -> Result<(), String> {
    let root = extracted.join("aether-linux-x86_64");
    let bin_dir = platform::bin_dir();

    install_binary(&root.join("aether"), &bin_dir.join("aether"))?;
    install_binary(&root.join("aetherd"), &bin_dir.join("aetherd"))?;
    install_binary(&root.join("aether-setup"), &bin_dir.join("aether-setup"))?;

    // Update the systemd service file
    let service_src = root.join("assets/linux/aetherd.service");
    let service_dst = platform::service_dir().join("aetherd.service");

    fs::copy(&service_src, &service_dst).map_err(|e| e.to_string())?;

    Ok(())
}

fn install_binary(src: &Path, dst: &Path) -> Result<(), String> {
    fs::copy(src, dst).map_err(|e| e.to_string())?;

    fs::set_permissions(dst, fs::Permissions::from_mode(0o755)).map_err(|e| e.to_string())?;

    Ok(())
}
