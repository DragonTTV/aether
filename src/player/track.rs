use std::time::Duration;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: u64,
    pub source: String,
    pub metadata: Metadata,
}
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Metadata{
    pub title: Option<String>,
    pub artist: Option<String>,
    pub duration: Option<Duration>,
    pub album: Option<String>,
    pub artwork: Option<String>,
    pub release_date: Option<String>,
}

impl Track{
    pub fn new(source: String) -> Self{
        let metadata = crate::audio::metadata::extract_metadata(&source)
            .unwrap_or_default();
        Self{
            id: 0,
            source,
            metadata,
        }
    } 
    pub fn display_name(&self) -> &str {
        self.metadata.title.as_deref().unwrap_or_else(|| {
            Path::new(&self.source)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(&self.source)
        })
    }
    pub fn formatted_duration(&self) -> Option<String> {
        self.metadata.duration.map(|duration| {
            let total = duration.as_secs();
            let minutes = total / 60;
            let seconds = total % 60;

            format!("{minutes:02}:{seconds:02}")
        })
    }
}