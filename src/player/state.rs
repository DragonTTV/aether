//state.rs

/// Represents the current playback state.
#[derive(PartialEq)]
pub enum PlaybackState{
    Playing,
    Paused,
    Stopped
}