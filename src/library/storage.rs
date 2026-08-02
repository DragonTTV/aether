use std::{fs, path::PathBuf};

use super::Library;

fn library_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap();

    path.push("aether");
    std::fs::create_dir_all(&path).ok();

    path.push("library.json");

    path
}

pub fn save(library: &Library) -> Result<(), String> {
    println!("Saving library...");
    let json = serde_json::to_string_pretty(library).map_err(|e| e.to_string())?;

    fs::write(library_path(), json).map_err(|e| e.to_string())
}

pub fn load() -> Result<Library, String> {
    let path = library_path();

    if !path.exists() {
        return Ok(Library::new());
    }

    let json = fs::read_to_string(path).map_err(|e| e.to_string())?;

    serde_json::from_str(&json).map_err(|e| e.to_string())
}

pub fn mark_as_migrated() -> Result<(), String> {
    let path = library_path();

    if !path.exists() {
        return Ok(());
    }

    let migrated_path = path.with_extension("json.migrated");

    fs::rename(path, migrated_path).map_err(|e| e.to_string())
}
