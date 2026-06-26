use std::time::Duration;

pub struct Track {
    pub source: String,
    pub metadata: Metadata,
}

pub struct Metadata{
    pub title: Option<String>,
    pub artist: Option<String>,
    pub duration: Option<Duration>,
    pub album: Option<String>,
    pub artwork: Option<String>,
    pub release_date: Option<String>,
}