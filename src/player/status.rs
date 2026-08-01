use crate::player::{PlaybackState, Track};

pub struct PlayerStatus {
    pub state: PlaybackState,
    pub volume: u8,
    pub current_track: Option<Track>,
    pub current_index: Option<usize>,
    pub queue: Vec<Track>,
}
