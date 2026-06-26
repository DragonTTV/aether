use crate::player::Track;

use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, DeviceSinkBuilder, Player};

pub struct AudioEngine{
    device: rodio::MixerDeviceSink,
    player: rodio::Player,
}

impl AudioEngine{
    pub fn new() -> Self{
        let device = DeviceSinkBuilder::open_default_sink().unwrap();
        let player = Player::connect_new(device.mixer());
        Self{device, player}
    }

    pub fn play(&self, track: &Track){
        let file = File::open(&track.source).unwrap();
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).unwrap();
        
        self.player.append(source);
        self.player.play();
    }
}