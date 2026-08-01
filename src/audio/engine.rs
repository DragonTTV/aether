use crate::player::Track;

use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, DeviceSinkBuilder, Player};

pub struct AudioEngine {
    #[allow(dead_code)]
    device: rodio::MixerDeviceSink,
    player: rodio::Player,
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioEngine {
    pub fn new() -> Self {
        let device = DeviceSinkBuilder::open_default_sink().unwrap();
        let player = Player::connect_new(device.mixer());

        Self { device, player }
    }
    pub fn is_finished(&self) -> bool {
        self.player.empty()
    }
    pub fn play(&self, track: &Track) {
        let file = File::open(&track.source).unwrap();
        let reader = BufReader::new(file);
        let source = Decoder::new(reader).unwrap();

        self.player.append(source);
        self.player.play();
    }
    pub fn pause(&self) {
        self.player.pause();
    }

    pub fn resume(&self) {
        self.player.play();
    }

    pub fn stop(&self) {
        self.player.stop();
        self.player.sleep_until_end();
    }

    pub fn set_volume(&self, volume: f32) {
        self.player.set_volume(volume);
    }
}
