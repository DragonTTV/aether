use crate::player::{Track};
pub struct Queue{
    pub tracks: Vec<Track>,
    pub current_index: Option<usize>, 
}

impl Queue {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            current_index: None,
        }
    }
    pub fn set_current(&mut self, index: usize) {
        self.current_index = Some(index);
    }
    pub fn clear_current(&mut self) {
        self.current_index = None;
    }
    pub fn add(&mut self, track: Track)->usize{
        self.tracks.push(track);
        self.tracks.len() - 1
    }
    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        if index >= self.tracks.len() {
            return Err("Index out of bounds".to_string());
        }

        if let Some(current) = self.current_index {
            if index == current {
                return Err("Cannot remove the currently playing track.".to_string());
            }
        }

        self.tracks.remove(index);

        if self.tracks.is_empty() {
            self.current_index = None;
        } else if let Some(current) = self.current_index {
            if index < current {
                self.current_index = Some(current - 1);
            }
        }

        Ok(())
    }
    pub fn current(&self) -> Option<&Track>{
        self.current_index.and_then(|index| self.tracks.get(index))
    }
    pub fn is_empty(&self) -> bool {
       self.tracks.is_empty()
    }
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }
    
    pub fn clear(&mut self){
        self.tracks.clear();
        self.current_index = None;
    }

    pub fn clear_upcoming(&mut self){
        let Some(current) = self.current_index else {
            self.clear();
            return;
        };

        let current_track = self.tracks[current].clone();

        self.tracks.clear();
        self.tracks.push(current_track);
        self.current_index = Some(0);
    }

    pub fn next(&mut self) -> bool{
        if let Some(current) = self.current_index{
            if current == self.tracks.len() - 1 {
                // self.current_index = None;
                return false;
            }
            self.current_index = Some(current + 1);
            return true;
            
        }
        false
    }
    pub fn previous(&mut self) -> bool {
        if let Some(current) = self.current_index{
            if current == 0{
                return false;
            }
            self.current_index = Some(current - 1);
            return true;
        }
        false
    }
}