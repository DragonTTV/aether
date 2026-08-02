use std::fmt;
/// Represents the current playback state.
#[derive(Debug, PartialEq, Clone)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}
/// Controls what happens when the current track finishes playing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    Off,
    Track,
    Queue,
}

impl fmt::Display for RepeatMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepeatMode::Off => write!(f, "Off"),
            RepeatMode::Track => write!(f, "Track"),
            RepeatMode::Queue => write!(f, "Queue"),
        }
    }
}
//Handles update event.
pub enum UpdateEvent {
    None,
    TrackChanged,
    PlaybackStopped
}