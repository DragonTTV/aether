use crate::{
    audio::AudioEngine,
    player::{PlaybackState, PlayerStatus, Queue, Track, RepeatMode},
};
pub struct Player {
    pub state: PlaybackState,
    pub queue: Queue,
    pub volume: u8,
    pub audio: AudioEngine,
    pub repeat: RepeatMode,
}
#[derive(Debug)]
pub enum PlayerError {
    AlreadyPlaying,
    AlreadyPaused,
    AlreadyStopped,
    NothingPlaying,
    QueueEmpty,
    EndOfQueue,
    BeginningOfQueue,
}
impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerError::AlreadyPaused => {
                write!(f, "Playback is already paused.")
            }
            PlayerError::AlreadyPlaying => {
                write!(f, "Playback is already playing.")
            }
            PlayerError::AlreadyStopped => {
                write!(f, "Playback is already stopped.")
            }
            PlayerError::NothingPlaying => {
                write!(f, "Nothing is currently playing.")
            }
            PlayerError::QueueEmpty => {
                write!(f, "Queue is empty.")
            }
            PlayerError::EndOfQueue => {
                write!(f, "Reached the end of the queue.")
            }
            PlayerError::BeginningOfQueue => {
                write!(f, "Already at the beginning of the queue.")
            }
        }
    }
}
pub struct PlaybackOutcome {
    pub started_playing: bool,
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
impl Player {
    pub fn new() -> Self {
        Self {
            state: PlaybackState::Stopped,
            queue: Queue::new(),
            volume: 50,
            audio: AudioEngine::new(),
            repeat: RepeatMode::Off,
        }
    }
    pub fn update(&mut self) {
        if self.state != PlaybackState::Playing {
            return;
        }
        if self.audio.is_finished() {
            match self.repeat {
                RepeatMode::Off => {
                    let _ = self.next_track();
                }
                RepeatMode::Track => {
                    let _ = self.restart_current();
                }
                RepeatMode::Queue => {
                    if self.next_track().is_err() {
                        self.queue.set_current(0);
                        let _ = self.restart_current();
                    }
                }
            }
        }
    }
    pub fn play(&mut self, track: Track) -> PlaybackOutcome {
        let started_playing = self.state == PlaybackState::Stopped;
        let index = self.queue.add(track);
        if started_playing {
            self.queue.set_current(index);
            let track = self.queue.current().unwrap();
            self.audio.play(track);
            self.state = PlaybackState::Playing;
        }
        PlaybackOutcome { started_playing }
    }
    pub fn pause(&mut self) -> Result<(), PlayerError> {
        match self.state {
            PlaybackState::Playing => {
                self.audio.pause();
                self.state = PlaybackState::Paused;
                Ok(())
            }
            PlaybackState::Paused => Err(PlayerError::AlreadyPaused),
            PlaybackState::Stopped => Err(PlayerError::NothingPlaying),
        }
    }
    pub fn resume(&mut self) -> Result<(), PlayerError> {
        match self.state {
            PlaybackState::Paused => {
                self.audio.resume();
                self.state = PlaybackState::Playing;
                Ok(())
            }
            PlaybackState::Playing => Err(PlayerError::AlreadyPlaying),
            PlaybackState::Stopped => {
                let track = self.queue.current().ok_or(PlayerError::NothingPlaying)?;

                self.audio.play(track);
                self.state = PlaybackState::Playing;

                Ok(())
            }
        }
    }
    pub fn stop(&mut self) -> Result<(), PlayerError> {
        match self.state {
            PlaybackState::Playing | PlaybackState::Paused => {
                self.audio.stop();
                self.state = PlaybackState::Stopped;
                Ok(())
            }
            PlaybackState::Stopped => Err(PlayerError::AlreadyStopped),
        }
    }
    pub fn next_track(&mut self) -> Result<&Track, PlayerError> {
        if self.queue.advance() {
            self.restart_current()
        } else {
            self.audio.stop();
            self.state = PlaybackState::Stopped;

            Err(PlayerError::EndOfQueue)
        }
    }
    pub fn previous_track(&mut self) -> Result<&Track, PlayerError> {
        if self.queue.go_back() {
            self.audio.stop();

            let track = self.queue.current().ok_or(PlayerError::QueueEmpty)?;

            self.audio.play(track);
            self.state = PlaybackState::Playing;

            Ok(track)
        } else {
            if self.queue.is_empty() {
                Err(PlayerError::QueueEmpty)
            } else {
                Err(PlayerError::BeginningOfQueue)
            }
        }
    }
    pub fn set_volume(&mut self, level: u8) {
        let level = level.clamp(0, 100);

        self.audio.set_volume(level as f32 / 100.0);
        self.volume = level;
    }
    pub fn get_volume(&self) -> u8 {
        self.volume
    }
    pub fn clear_queue(&mut self) {
        self.queue.clear_upcoming();
    }
    pub fn status(&self) -> PlayerStatus {
        PlayerStatus {
            state: self.state.clone(),
            volume: self.volume,
            current_track: self.current_track().cloned(),
            current_index: self.queue.current_index(),
            queue: self.queue.tracks().to_vec(),
        }
    }
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
    pub fn current_track(&self) -> Option<&Track> {
        self.queue.current()
    }
    pub fn remove_from_queue(&mut self, index: usize) -> Result<(), String> {
        self.queue.remove(index)
    }
    pub fn play_now(&mut self, track: Track) -> PlaybackOutcome {
        self.audio.stop();

        self.queue.replace_current_and_upcoming(track);

        let track = self.queue.current().unwrap();

        self.audio.play(track);
        self.state = PlaybackState::Playing;

        PlaybackOutcome {
            started_playing: true,
        }
    }
    pub fn play_all(&mut self, tracks: Vec<Track>) -> PlaybackOutcome {
        let mut started_playing = false;

        for track in tracks {
            let outcome = self.play(track);

            if outcome.started_playing {
                started_playing = true;
            }
        }

        PlaybackOutcome { started_playing }
    }

    pub fn play_all_now(&mut self, tracks: Vec<Track>) -> PlaybackOutcome {
        if tracks.is_empty() {
            return PlaybackOutcome {
                started_playing: false,
            };
        }

        self.audio.stop();
        self.queue.replace_current_and_upcoming_many(tracks);

        let track = self.queue.current().unwrap();

        self.audio.play(track);
        self.state = PlaybackState::Playing;

        PlaybackOutcome {
            started_playing: true,
        }
    }
    pub fn set_repeat(&mut self, mode: RepeatMode) {
        self.repeat = mode;
    }

    pub fn repeat(&self) -> RepeatMode {
        self.repeat
    }
    pub fn restart_current(&mut self) -> Result<&Track, PlayerError> {
        self.audio.stop();

        let track = self.queue.current().ok_or(PlayerError::QueueEmpty)?;
        self.audio.play(track);
        self.state = PlaybackState::Playing;
        Ok(track)
    }
}
