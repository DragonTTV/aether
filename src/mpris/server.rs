use std::sync::{Arc, Mutex};

use zbus::{connection::Builder, Connection};

use crate::{
    mpris::{
        player::PlayerInterface, root::RootInterface,
    }, player::Player,
};

pub struct MprisServer {
    connection: Arc<Connection>,
}

const MPRIS_PATH: &str = "/org/mpris/MediaPlayer2";

impl MprisServer {
    pub async fn new(player: Arc<Mutex<Player>>) -> zbus::Result<Self> {
        let connection = Arc::new(
            Builder::session()?
                .name("org.mpris.MediaPlayer2.aether")?
                .serve_at(
                    MPRIS_PATH,
                    RootInterface,
                )?
                .serve_at(
                    MPRIS_PATH,
                    PlayerInterface::new(player),
                )?
                .build()
                .await?,
        );

        Ok(Self { connection })
    }

    pub fn connection(&self) -> Arc<Connection> {
        self.connection.clone()
    }

    pub async fn notify_metadata(&self) -> zbus::Result<()> {
        let iface_ref = self
            .connection
            .object_server()
            .interface::<_, PlayerInterface>(MPRIS_PATH)
            .await?;

        let iface = iface_ref.get().await;

        iface
            .metadata_changed(iface_ref.signal_emitter())
            .await
    }
    pub async fn notify_playback_status(&self) -> zbus::Result<()> {
        let iface_ref = self
            .connection
            .object_server()
            .interface::<_, PlayerInterface>(MPRIS_PATH)
            .await?;

        iface_ref
            .get()
            .await
            .playback_status_changed(iface_ref.signal_emitter())
            .await
    }

    pub async fn notify_position(&self) -> zbus::Result<()> {
        let iface_ref = self
            .connection
            .object_server()
            .interface::<_, PlayerInterface>(MPRIS_PATH)
            .await?;

        iface_ref
            .get()
            .await
            .position_changed(iface_ref.signal_emitter())
            .await
    }
}