//state.rs

/// Represents the current playback state.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PlaybackState{
    Playing,
    Paused,
    Stopped
}