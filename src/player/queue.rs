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

    pub fn add(&mut self, track: Track){
        self.tracks.push(track);
        if self.current_index.is_none(){
            self.current_index = Some(0);
        }
    }
    pub fn remove(&mut self, index: usize)-> Result<(), String>{
        if index >= self.tracks.len(){
            return Err("Index out of bounds".to_string());
        }
        self.tracks.remove(index);

        if self.tracks.is_empty(){
                self.current_index = None;
        } else if let Some(current) = self.current_index{
            if index == current{
                self.current_index = Some(0);
            }else if index < current{
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
    pub fn next(&mut self) -> bool{
        if let Some(current) = self.current_index{
            if current == self.tracks.len()-1{
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