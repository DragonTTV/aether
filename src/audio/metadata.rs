use crate::player::Metadata;
use lofty::prelude::*;
use lofty::probe::Probe;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

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
        let picture = tag
            .pictures()
            .iter()
            .find(|p| p.pic_type() == lofty::picture::PictureType::CoverFront)
            .or_else(|| tag.pictures().first());

        if let Some(picture) = picture {
            let mut artwork_dir = dirs::data_local_dir().unwrap();
            artwork_dir.push("aether");
            artwork_dir.push("artwork");

            fs::create_dir_all(&artwork_dir).map_err(|e| e.to_string())?;

            let mut artwork_path = artwork_dir.clone();
            let hash = Sha256::digest(path.as_bytes());
            let extension = match picture.mime_type() {
                Some(lofty::picture::MimeType::Jpeg) => "jpg",
                Some(lofty::picture::MimeType::Png) => "png",
                _ => "img",
            };
            let filename = format!("{}.{}", hex::encode(hash), extension);

            artwork_path.push(filename);

            if !artwork_path.exists() {
                fs::write(&artwork_path, picture.data()).map_err(|e| e.to_string())?;
            }

            metadata.artwork = Some(format!("file://{}", artwork_path.display()));
        }
        metadata.title = tag.title().map(|s| s.into_owned());
        metadata.artist = tag.artist().map(|s| s.into_owned());
        metadata.album = tag.album().map(|s| s.into_owned());

        metadata.genre = tag.genre().map(|s| s.into_owned());

        metadata.track_number = tag.track();
        metadata.disc_number = tag.disk();
    }

    Ok(metadata)
}
