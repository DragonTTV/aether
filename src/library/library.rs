use crate::player::Track;
use serde::{Deserialize, Serialize};
use crate::cli::SortBy;
#[derive(Default, Serialize, Deserialize)]
pub struct Library {
    next_id: u64,
    tracks: Vec<Track>,
    scan_paths: Vec<String>,
}

impl Library {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            tracks: Vec::new(),
            scan_paths: Vec::new()
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
        self.next_id = 1;
        // scan_paths stays intact
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
    pub fn sorted_tracks(&self, sort: Option<SortBy>) -> Vec<&Track>{
        let mut tracks: Vec<&Track> = self.tracks.iter().collect();
        match sort {
            Some(SortBy::Title) => {
                tracks.sort_by_key(|track| {
                    track.display_name().to_lowercase()
                });
            }
            Some(SortBy::Artist) => {
                tracks.sort_by_key(|track| {
                    track.metadata
                        .artist
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                });
            }
            Some(SortBy::Album) => {
                tracks.sort_by_key(|track| {
                    track.metadata
                        .album
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                });
            }
            None => {}
        }
        tracks
    }
    pub fn add_scan_path(&mut self, path: String) {
        if !self.scan_paths.contains(&path) {
            self.scan_paths.push(path);
        }
    }
    pub fn scan_paths(&self) -> &[String] {
        &self.scan_paths
    }
}