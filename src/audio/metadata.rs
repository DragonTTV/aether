use std::path::{Path, PathBuf};
use std::fs;
use lofty::prelude::*;
use lofty::probe::Probe;
use sha2::{Digest,Sha256};
use crate::player::Metadata;

pub fn extract_metadata(path: &str) -> Result<Metadata, String> {
    println!("extract_metadata: {}", path);
    let tagged_file = Probe::open(Path::new(path))
        .map_err(|e| e.to_string())?
        .read()
        .map_err(|e| e.to_string())?;

    let properties = tagged_file.properties();

    let mut metadata = Metadata {
        duration: Some(properties.duration()),
        ..Default::default()
    };

    if let Some(tag) = tagged_file.primary_tag() {
        println!("Pictures: {}", tag.pictures().len());
        if let Some(picture) = tag.pictures().first() {
            let mut artwork_dir = dirs::data_local_dir().unwrap();
            artwork_dir.push("aether");
            artwork_dir.push("artwork");

            fs::create_dir_all(&artwork_dir)
                .map_err(|e| e.to_string())?;

            let mut artwork_path = artwork_dir.clone();
            let hash = Sha256::digest(path.as_bytes());
            let filename = format!("{}.jpg", hex::encode(hash));

            artwork_path.push(filename);

            fs::write(&artwork_path, picture.data())
                .map_err(|e| e.to_string())?;

            println!("Saved artwork to {}", artwork_path.display());

            metadata.artwork = Some(format!("file://{}", artwork_path.display()));
        }
        metadata.title = tag.title().map(|s| s.into_owned());
        metadata.artist = tag.artist().map(|s| s.into_owned());
        metadata.album = tag.album().map(|s| s.into_owned());
    }

    Ok(metadata)
}
