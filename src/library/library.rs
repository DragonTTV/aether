use crate::player::Track;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Library {
    next_id: u64,
    tracks: Vec<Track>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            tracks: Vec::new(),
        }
    }

    pub fn add(&mut self, mut track: Track) {
        track.id = self.next_id;
        self.next_id += 1;
        self.tracks.push(track);
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }
    pub fn get(&self, id: u64) -> Option<&Track> {
        self.tracks.iter().find(|track| track.id == id)
    }
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }
    pub fn clear(&mut self) {
       self.tracks.clear();
    }
    pub fn search(&self, query: &str) -> Vec<&Track>{
        let query = query.to_lowercase();
        self.tracks
        .iter()
        .filter(|track| {
            track.metadata
                .title
                .as_deref()
                .unwrap_or("")
                .to_lowercase()
                .contains(&query)
                ||
            track.metadata
                .artist
                .as_deref()
                .unwrap_or("")
                .to_lowercase()
                .contains(&query)
                ||
            track.metadata
                .album
                .as_deref()
                .unwrap_or("")
                .to_lowercase()
                .contains(&query)
                ||
            track.display_name()
                .to_lowercase()
                .contains(&query)
        })
        .collect()
    }
}