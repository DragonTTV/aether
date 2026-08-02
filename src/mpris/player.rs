use std::sync::{Arc, Mutex};
use zbus::interface;

use crate::player::{PlaybackState, Player, RepeatMode};
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
            if let Some(genre) = &track.metadata.genre {
                metadata.insert(
                    "xesam:genre".into(),
                    Value::from(vec![genre.clone()]),
                );
            }

            if let Some(track_number) = track.metadata.track_number {
                metadata.insert(
                    "xesam:trackNumber".into(),
                    Value::from(track_number as i32),
                );
            }

            if let Some(disc_number) = track.metadata.disc_number {
                metadata.insert(
                    "xesam:discNumber".into(),
                    Value::from(disc_number as i32),
                );
            }

            metadata.insert(
                "xesam:url".into(),
                Value::from(format!("file://{}", track.source)),
            );

            if let Some(date) = &track.metadata.release_date {
                metadata.insert(
                    "xesam:contentCreated".into(),
                    Value::from(date.clone()),
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

    #[zbus(property)]
    fn loop_status(&self) -> String {
        let player = self.player.lock().unwrap();

        match player.repeat() {
            RepeatMode::Off => "None",
            RepeatMode::Track => "Track",
            RepeatMode::Queue => "Playlist",
        }
        .to_string()
    }

    #[zbus(property)]
    fn set_loop_status(&self, status: String) {
        let mut player = self.player.lock().unwrap();

        let mode = match status.as_str() {
            "Track" => RepeatMode::Track,
            "Playlist" => RepeatMode::Queue,
            _ => RepeatMode::Off,
        };

        player.set_repeat(mode);
    }

    #[zbus(property)]
    fn shuffle(&self) -> bool {
        self.player.lock().unwrap().shuffle()
    }

    #[zbus(property)]
    fn set_shuffle(&self, enabled: bool) {
        self.player.lock().unwrap().set_shuffle(enabled);
    }

    #[zbus(property)]
    fn volume(&self) -> f64 {
        self.player.lock().unwrap().get_volume() as f64 / 100.0
    }

    #[zbus(property)]
    fn set_volume(&self, volume: f64) {
        let level = (volume * 100.0).clamp(0.0, 100.0) as u8;
        let _ = self.player.lock().unwrap().set_volume(level);
    }
}