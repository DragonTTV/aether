use crate::{audio::AudioEngine, player::{PlaybackState, Queue, Track, PlayerStatus}};
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
            let _ = self.next();
        }
    }
    pub fn play(&mut self, track:Track){

        let index = self.queue.add(track);
        if self.state == PlaybackState::Stopped{
            self.queue.set_current(index);
            let track = self.queue.current().unwrap();

            self.audio.play(track);
            self.state = PlaybackState::Playing;
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
    pub fn next(&mut self) -> Result<(), String>{
        if self.queue.next(){
            self.audio.stop();

            if let Some(track) = self.queue.current(){
                self.audio.play(track);
                self.state = PlaybackState::Playing;
                return Ok(());
            }
            return Err("No current track.".to_string());
        }   
            self.queue.clear_current();
            self.state = PlaybackState::Stopped;
            Err("End of queue.".to_string())
        
    }
    pub fn previous(&mut self) -> Result<(), String>{
        if self.queue.previous(){
            self.audio.stop();
            let Some(track) = self.queue.current() else{
                return Err("No current track".to_string());
            };

            self.audio.play(track);
            self.state = PlaybackState::Playing;
            return Ok(());
        }
        Err("Already at the beginning of the queue.".to_string())
    }
    pub fn set_volume(&mut self, level: u8) {
        let level = level.clamp(0, 100);

        self.audio.set_volume(level as f32 / 100.0);
        self.volume = level;
    }
    pub fn get_volume(&self) -> u8{
        self.volume
    }
    pub fn clear_queue(&mut self){
        self.queue.clear_upcoming();
    }
    pub fn status(&self) -> PlayerStatus{
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
}