use std::sync::{Arc, Mutex};
use zbus::interface;

use crate::player::{PlaybackState, Player};
use std::collections::HashMap;
use zvariant::{ObjectPath, Value};
pub struct PlayerInterface {
    player: Arc<Mutex<Player>>,
}

impl PlayerInterface {
    pub fn new(player: Arc<Mutex<Player>>) -> Self {
        Self { player }
    }
}

#[interface(name = "org.mpris.MediaPlayer2.Player")]
impl PlayerInterface {
    #[zbus(property)]
    fn playback_status(&self) -> &'static str {
        let player = self.player.lock().unwrap();

        match player.state {
            PlaybackState::Playing => "Playing",
            PlaybackState::Paused => "Paused",
            PlaybackState::Stopped => "Stopped",
        }
    }

    #[zbus(property)]
    fn metadata(&self) -> HashMap<String, Value<'static>> {
        let player = self.player.lock().unwrap();

        let mut metadata = HashMap::new();

        if let Some(track) = player.current_track() {
            metadata.insert(
                "mpris:trackid".into(),
                Value::from(
                    ObjectPath::try_from("/org/mpris/MediaPlayer2/track/0").unwrap(),
                ),
            );

            metadata.insert(
                "xesam:title".into(),
                Value::from(track.display_name().to_string()),
            );

            if let Some(artist) = &track.metadata.artist {
                metadata.insert(
                    "xesam:artist".into(),
                    Value::from(vec![artist.clone()]),
                );
            }

            if let Some(album) = &track.metadata.album {
                metadata.insert(
                    "xesam:album".into(),
                    Value::from(album.clone()),
                );
            }

            if let Some(duration) = track.metadata.duration {
                metadata.insert(
                    "mpris:length".into(),
                    Value::from(duration.as_micros() as i64),
                );
            }
            if let Some(artwork) = &track.metadata.artwork {
                metadata.insert(
                    "mpris:artUrl".into(),
                    Value::from(artwork.clone()),
                );
            }
        }

        metadata
    }
        #[zbus(property)]
    fn can_control(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_play(&self) -> bool {
        self.player.lock().unwrap().current_track().is_some()
    }

    #[zbus(property)]
    fn can_pause(&self) -> bool {
        self.player.lock().unwrap().current_track().is_some()
    }

    #[zbus(property)]
    fn can_go_next(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_go_previous(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_seek(&self) -> bool {
        true
    }
    #[zbus(property)]
    fn minimum_rate(&self) -> f64 {
        1.0
    }

    #[zbus(property)]
    fn maximum_rate(&self) -> f64 {
        1.0
    }

    #[zbus(property)]
    fn rate(&self) -> f64 {
        1.0
    }
    fn next(&self) {
        println!("MPRIS: Next");
        let mut player = self.player.lock().unwrap();
        let _ = player.next_track();
    }

    fn previous(&self) {
        println!("MPRIS: Previous");
        let mut player = self.player.lock().unwrap();
        let _ = player.previous_track();
    }

    async fn pause(&self,#[zbus(signal_emitter)] emitter: zbus::object_server::SignalEmitter<'_>,) {
        println!("MPRIS Pause");

        {
            let mut player = self.player.lock().unwrap();
            let _ = player.pause();
        }

        let _ = self.playback_status_changed(&emitter).await;
    }

    async fn play(&self, #[zbus(signal_emitter)] emitter: zbus::object_server::SignalEmitter<'_>,) {
        println!("MPRIS: Play");
        {
        let mut player = self.player.lock().unwrap();
        let _ = player.resume();
        }
        let _ = self.playback_status_changed(&emitter).await;
    }

    fn stop(&self) {
        println!("MPRIS: Stop");
        let mut player = self.player.lock().unwrap();
        let _ = player.stop();
    }

    fn play_pause(&self) {
        println!("MPRIS: Play-Pause");
        let mut player = self.player.lock().unwrap();

        match player.state {
            PlaybackState::Playing => {
                let _ = player.pause();
            }
            PlaybackState::Paused | PlaybackState::Stopped => {
                let _ = player.resume();
            }
        }
    }
    #[zbus(property)]
    fn position(&self) -> i64 {
        let player = self.player.lock().unwrap();
        player.position().as_micros() as i64
    }

    fn seek(&self, offset: i64) {
        let player = self.player.lock().unwrap();

        let current = player.position().as_micros() as i64;
        let new_position = (current + offset).max(0);

        let _ = player.seek(std::time::Duration::from_micros(
            new_position as u64,
        ));
    }

    fn set_position(&self, _track_id: ObjectPath<'_>, position: i64) {
        let player = self.player.lock().unwrap();

        let position = position.max(0);

        let _ = player.seek(std::time::Duration::from_micros(
            position as u64,
        ));
    }
}