use crate::player::Track;
pub struct Queue {
    pub tracks: Vec<Track>,
    pub current_index: Option<usize>,
}
impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn add(&mut self, track: Track) -> usize {
        self.tracks.push(track);
        self.tracks.len() - 1
    }
    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        if index >= self.tracks.len() {
            return Err("Index out of bounds".to_string());
        }

        if let Some(current) = self.current_index {
            if index <= current {
                return Err("Cannot remove the current track or playback history.".to_string());
            }
        } else {
            return Err("There are no upcoming tracks to remove.".to_string());
        }

        self.tracks.remove(index);

        Ok(())
    }
    pub fn current(&self) -> Option<&Track> {
        self.current_index.and_then(|index| self.tracks.get(index))
    }
    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }
    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.current_index = None;
    }

    pub fn clear_upcoming(&mut self) {
        let Some(current) = self.current_index else {
            self.clear();
            return;
        };

        self.tracks.truncate(current + 1);
    }

    pub fn advance(&mut self) -> bool {
        if let Some(current) = self.current_index {
            if current == self.tracks.len() - 1 {
                self.current_index = None;
                return false;
            }
            self.current_index = Some(current + 1);
            return true;
        }
        false
    }
    pub fn go_back(&mut self) -> bool {
        match self.current_index {
            Some(current) => {
                if current == 0 {
                    return false;
                }
                self.current_index = Some(current - 1);
                true
            }
            None => {
                if self.tracks.is_empty() {
                    return false;
                }
                self.current_index = Some(self.tracks.len() - 1);
                true
            }
        }
    }
    pub fn replace_current_and_upcoming(&mut self, track: Track) {
        match self.current_index {
            Some(current) => {
                self.tracks.truncate(current + 1);
                self.tracks.push(track);
                self.current_index = Some(current + 1);
            }
            None => {
                self.tracks.push(track);
                self.current_index = Some(self.tracks.len() - 1);
            }
        }
    }
    pub fn add_many(&mut self, tracks: Vec<Track>) {
        self.tracks.extend(tracks);
    }
    pub fn replace_current_and_upcoming_many(&mut self, tracks: Vec<Track>) {
        if tracks.is_empty() {
            return;
        }

        match self.current_index {
            Some(current) => {
                self.tracks.truncate(current + 1);

                let new_current = self.tracks.len();
                self.tracks.extend(tracks);
                self.current_index = Some(new_current);
            }
            None => {
                let new_current = self.tracks.len();
                self.tracks.extend(tracks);
                self.current_index = Some(new_current);
            }
        }
    }

}
