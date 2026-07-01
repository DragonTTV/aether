//state.rs

/// Represents the current playback state.
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum PlaybackState{
    Playing,
    Paused,
    Stopped
}