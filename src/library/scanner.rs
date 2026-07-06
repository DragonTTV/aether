use std::path::Path;
use walkdir::WalkDir;
use super::Library;
use crate:: player::Track;



pub fn scan(path: &Path, library: &mut Library) -> Result<(), String> {
    let scan_path = path.to_string_lossy().into_owned();
    
    library.add_scan_path(scan_path.clone());

    for entry in WalkDir::new(&scan_path) {
        let entry = entry.map_err(|e| e.to_string())?;

        if !entry.file_type().is_file() {
            continue;
        }

        if is_supported(entry.path()) {
            let track = Track::new(entry.path().to_string_lossy().into_owned(),);
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