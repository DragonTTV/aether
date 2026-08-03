use crate::{platform, setup::ui};

pub fn doctor() -> Result<(), String> {
    ui::header();

    ui::step("Checking environment...");
    check_environment()?;
    ui::newline();

    ui::step("Checking installation...");
    check_installation()?;
    ui::newline();

    ui::step("Checking service...");
    check_service()?;
    ui::newline();

    ui::step("Checking runtime...");
    check_runtime()?;
    ui::newline();

    ui::step("Checking storage...");
    check_storage()?;
    ui::newline();

    ui::success("No issues found.");

    Ok(())
}

fn check_environment() -> Result<(), String> {
    if std::process::Command::new("systemctl")
        .arg("--version")
        .output()
        .is_ok()
    {
        ui::success("systemd");
    } else {
        ui::error("systemd not found");
    }

    Ok(())
}

fn check_installation() -> Result<(), String> {
    let bin_dir = platform::bin_dir();

    check_binary(&bin_dir.join("aether"), "aether");
    check_binary(&bin_dir.join("aetherd"), "aetherd");
    check_binary(&bin_dir.join("aether-setup"), "aether-setup");

    Ok(())
}

fn check_service() -> Result<(), String> {
    ui::info("Service checks not implemented yet.");
    Ok(())
}

fn check_runtime() -> Result<(), String> {
    ui::info("Runtime checks not implemented yet.");
    Ok(())
}

fn check_storage() -> Result<(), String> {
    let data_dir = platform::data_dir();
    let artwork_dir = platform::artwork_dir();

    if data_dir.exists() {
        ui::success("Data directory");
    } else {
        ui::error("Data directory missing");
    }

    if artwork_dir.exists() {
        ui::success("Artwork cache");
    } else {
        ui::warning("Artwork cache missing");
    }

    Ok(())
}

fn check_binary(path: &std::path::Path, name: &str) {
    if path.exists() {
        ui::success(&format!("{name} binary found"));
    } else {
        ui::error(&format!("{name} binary missing"));
    }
}
