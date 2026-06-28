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
    pub fn update(&mut self) {
        if self.state != PlaybackState::Playing {
            return;
        }

        if self.audio.is_finished() {
            self.next();
        }
    }
    pub fn play(&mut self, track:Track){
        // println!("Playing: {}", track.source);
        // self.audio.play(&track);
        let should_start = self.queue.is_empty();
        self.queue.add(track);
        if should_start{
            if let Some(track) = self.queue.current(){
                self.audio.play(track);
                self.state = PlaybackState::Playing;
            }
        }
    }
    pub fn pause(&mut self){
        if self.state == PlaybackState::Playing {
            self.audio.pause();
            self.state = PlaybackState::Paused;
        }
    }
    pub fn resume(&mut self){
        if self.state == PlaybackState::Paused {
            self.audio.resume();
            self.state = PlaybackState::Playing;
        }
    }
    pub fn stop(&mut self){
        self.audio.stop();
        self.state = PlaybackState::Stopped;
        // self.queue.current_index = None;
    }
    pub fn next(&mut self){
        if self.queue.next(){
            self.audio.stop();
            if let Some(track) = self.queue.current(){
                self.audio.play(track);
                self.state = PlaybackState::Playing;
            }
        }else {
            self.state = PlaybackState::Stopped;
        }
    }
    pub fn previous(&mut self) {
        if self.queue.previous(){
            self.audio.stop();
            if let Some(track) = self.queue.current(){
                self.audio.play(track);
                self.state = PlaybackState::Playing;
            }
        }else {
            self.state = PlaybackState::Stopped;
        }
    }
    pub fn set_volume(&mut self, level: u8) {
        self.audio.set_volume(level as f32/100.0);
        self.volume = level.min(100);
    }
    pub fn status(&self){
        todo!();
    }
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
}