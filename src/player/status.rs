use std::time::Duration;

use crate::player::{PlaybackState, RepeatMode, Track};

pub struct PlayerStatus {
    pub state: PlaybackState,
    pub volume: u8,
    pub current_track: Option<Track>,
    pub current_index: Option<usize>,
    pub queue: Vec<Track>,
    pub repeat: RepeatMode,
    pub shuffle: bool,
    pub position: Duration,
    pub duration: Option<Duration>,
}
