use super::Library;
use crate::player::Track;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

pub fn scan(path: &Path, library: &mut Library) -> Result<(), String> {
    let scan_path = path.to_string_lossy().into_owned();

    library.add_scan_path(scan_path.clone());

    for entry in WalkDir::new(&scan_path) {
        let entry = entry.map_err(|e| e.to_string())?;

        if !entry.file_type().is_file() {
            continue;
        }

        if is_supported(entry.path()) {
            let track = Track::new(entry.path().to_string_lossy().into_owned());
            library.add(track)
        }
    }

    Ok(())
}

fn is_supported(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some("mp3")
            | Some("flac")
            | Some("wav")
            | Some("ogg")
            | Some("opus")
            | Some("m4a")
            | Some("aac")
    )
}

pub fn rescan(
    path: &Path,
    library: &mut Library,
    old_tracks: &HashMap<String, Track>,
) -> Result<(), String> {
    let scan_path = path.to_string_lossy().into_owned();

    for entry in WalkDir::new(&scan_path) {
        let entry = entry.map_err(|e| e.to_string())?;

        if !entry.file_type().is_file() {
            continue;
        }

        if is_supported(entry.path()) {
            let source = entry.path().to_string_lossy().into_owned();

            if let Some(track) = old_tracks.get(&source) {
                library.add_with_id(track.clone(), track.id);
            } else {
                let track = Track::new(source);
                library.add(track);
            }
        }
    }

    Ok(())
}

pub fn rescan_reid(
    path: &Path,
    library: &mut Library,
    old_tracks: &HashMap<String, Track>,
) -> Result<HashMap<u64, u64>, String> {
    let scan_path = path.to_string_lossy().into_owned();

    let mut id_map = HashMap::new();

    for entry in WalkDir::new(&scan_path) {
        let entry = entry.map_err(|e| e.to_string())?;

        if !entry.file_type().is_file() {
            continue;
        }

        if !is_supported(entry.path()) {
            continue;
        }

        let source = entry.path().to_string_lossy().into_owned();

        if let Some(track) = old_tracks.get(&source) {
            let old_id = track.id;
            let mut track = track.clone();
            track.id = 0;
            library.add(track);
            let new_id = library.tracks().last().unwrap().id;
            id_map.insert(old_id, new_id);
        } else {
            library.add(Track::new(source));
        }
    }
    Ok(id_map)
}
