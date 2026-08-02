use crate::player::Track;
use rodio::{Decoder, DeviceSinkBuilder, Player};
use std::fs::File;
use std::num::NonZero;
use std::{fmt, time::Duration};

#[derive(Debug)]
pub enum AudioError {
    SeekFailed,
    DecoderError,
    OutputDeviceUnavailable,
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::SeekFailed => write!(f, "Failed to seek playback."),
            AudioError::DecoderError => write!(f, "Audio decoder error."),
            AudioError::OutputDeviceUnavailable => {
                write!(f, "No audio output device available.")
            }
        }
    }
}

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
        let device = DeviceSinkBuilder::from_default_device()
            .unwrap()
            .with_sample_rate(NonZero::new(48000).unwrap())
            .open_sink_or_fallback()
            .unwrap();
        println!("Output config: {:?}", device.config());
        let player = Player::connect_new(device.mixer());

        Self { device, player }
    }
    pub fn is_finished(&self) -> bool {
        self.player.empty()
    }
    pub fn play(&self, track: &Track) {
        let file = File::open(&track.source).unwrap();
        let len = file.metadata().unwrap().len();

        let source = Decoder::builder()
            .with_data(file)
            .with_byte_len(len)
            .with_seekable(true)
            .with_gapless(true)
            .build()
            .unwrap();

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
        // self.player.sleep_until_end();
    }

    pub fn set_volume(&self, volume: f32) {
        self.player.set_volume(volume);
    }
    pub fn seek(&self, position: Duration) -> Result<(), AudioError> {
        match self.player.try_seek(position) {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("Seek error: {e:?}");
                Err(AudioError::SeekFailed)
            }
        }
    }
    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }
}
