pub struct Player{
    pub state: PlaybackState,
    pub queue: Queue,
    pub volume: u8,
}

impl Player{
    pub fn new() -> Self {
        Self{
            state: PlaybackState::Stopped,
            queue: Queue::new(),
            volume: 50,
        }
    }

    pub fn play(&mut self, track:Track){
        todo!()
    }
    pub fn pause(&mut self){
        todo!()
    }
    pub fn resume(&mut self){
        todo!()
    }
    pub fn stop(&mut self){
        todo!()
    }
    pub fn next(&mut self) {
        todo!()
    }
    pub fn previous(&mut self) {
        todo!()
    }
    pub fn set_volume(&mut self, level: u8) {
        todo!()
    }
}