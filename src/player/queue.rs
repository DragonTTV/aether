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
}