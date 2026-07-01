use std::path::Path;

use lofty::prelude::*;
use lofty::probe::Probe;

use crate::player::Metadata;

pub fn extract_metadata(path: &str) -> Result<Metadata, String> {
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
        metadata.title = tag.title().map(|s| s.into_owned());
        metadata.artist = tag.artist().map(|s| s.into_owned());
        metadata.album = tag.album().map(|s| s.into_owned());
    }

    Ok(metadata)
}