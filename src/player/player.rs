use crate::{audio::AudioEngine, player::{PlaybackState, Queue, Track}};
pub struct Player{
    pub state: PlaybackState,
    pub queue: Queue,
    pub volume: u8,
    pub audio: AudioEngine,
}

impl Player{
    pub fn new() -> Self {
        Self{
            state: PlaybackState::Stopped,
            queue: Queue::new(),
            volume: 50,
            audio: AudioEngine::new(),
        }
    }

    pub fn play(&mut self, track:Track){
        println!("Playing: {}", track.source);
        self.audio.play(&track);
        self.queue.add(track);
        self.state = PlaybackState::Playing;
    }
    pub fn pause(&mut self){
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }
    pub fn resume(&mut self){
        if self.state == PlaybackState::Paused {
            self.state = PlaybackState::Playing;
        }
    }
    pub fn stop(&mut self){
        self.state = PlaybackState::Stopped;
        // self.queue.current_index = None;
    }
    // pub fn next(&mut self) {
    //     todo!()
    // }
    // pub fn previous(&mut self) {
    //     todo!()
    // }
    pub fn set_volume(&mut self, level: u8) {
        self.volume = level.min(100);
    }
}