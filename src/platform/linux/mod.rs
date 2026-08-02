use std::path::PathBuf;

pub mod daemon;
pub mod process;

pub fn bin_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".local/bin")
}

pub fn data_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".local/share/aether")
}

pub fn artwork_dir() -> PathBuf {
    data_dir().join("artwork")
}

pub fn service_dir() -> PathBuf {
    dirs::home_dir().unwrap().join(".config/systemd/user")
}