use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::fdo::{Properties, Result};
use zbus::{interface, Connection};

use crate::icy::IcyMetadata;

type VoidCallback = Arc<Mutex<Option<Box<dyn Fn() + Send + Sync>>>>;
type StringCallback = Arc<Mutex<Option<Box<dyn Fn(String) + Send + Sync>>>>;
type FloatCallback = Arc<Mutex<Option<Box<dyn Fn(f64) + Send + Sync>>>>;

#[derive(Debug, Clone, PartialEq)]
pub struct MprisState {
    pub playing: bool,
    pub station_name: String,
    pub track_title: String,
    pub track_artist: Option<String>,
    pub url: String,
    pub volume: f64,
    pub muted: bool,
    pub icy_metadata: Option<IcyMetadata>,
    pub audio_bitrate: Option<u32>,
}

impl Default for MprisState {
    fn default() -> Self {
        Self {
            playing: false,
            station_name: String::new(),
            track_title: String::new(),
            track_artist: None,
            url: String::new(),
            volume: 80.0,
            muted: false,
            icy_metadata: None,
            audio_bitrate: None,
        }
    }
}

struct MediaPlayer2 {
    quit_callback: VoidCallback,
}

#[interface(name = "org.mpris.MediaPlayer2")]
impl MediaPlayer2 {
    #[zbus(property)]
    fn can_quit(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_raise(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn has_track_list(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn identity(&self) -> &str {
        "Radioboat"
    }

    #[zbus(property)]
    fn desktop_entry(&self) -> &str {
        "radioboat"
    }

    #[zbus(property)]
    fn supported_uri_schemes(&self) -> Vec<&str> {
        vec!["http", "https"]
    }

    #[zbus(property)]
    fn supported_mime_types(&self) -> Vec<&str> {
        vec!["audio/mpeg", "audio/aac", "audio/ogg", "audio/flac"]
    }

    fn raise(&self) -> Result<()> {
        Err(zbus::fdo::Error::NotSupported("Raise not supported".into()))
    }

    async fn quit(&self) -> Result<()> {
        if let Some(cb) = self.quit_callback.lock().await.as_ref() {
            cb();
        }
        Ok(())
    }
}

struct MediaPlayer2Player {
    state: Arc<Mutex<MprisState>>,
    play_callback: StringCallback,
    stop_callback: VoidCallback,
    volume_callback: FloatCallback,
    next_callback: VoidCallback,
    prev_callback: VoidCallback,
}

#[interface(name = "org.mpris.MediaPlayer2.Player")]
impl MediaPlayer2Player {
    #[zbus(property)]
    async fn playback_status(&self) -> String {
        let state = self.state.lock().await;
        if state.playing {
            "Playing".to_string()
        } else {
            "Stopped".to_string()
        }
    }

    #[zbus(property)]
    fn loop_status(&self) -> &str {
        "None"
    }

    #[zbus(property)]
    fn rate(&self) -> f64 {
        1.0
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
    async fn volume(&self) -> f64 {
        let state = self.state.lock().await;
        if state.muted {
            0.0
        } else {
            state.volume / 100.0
        }
    }

    #[zbus(property)]
    async fn set_volume(&self, value: f64) {
        let normalized = value.clamp(0.0, 1.0) * 100.0;
        self.state.lock().await.volume = normalized;
        if let Some(cb) = self.volume_callback.lock().await.as_ref() {
            cb(normalized);
        }
    }

    #[zbus(property)]
    fn position(&self) -> i64 {
        0
    }

    #[zbus(property)]
    async fn metadata(&self) -> HashMap<&'static str, zbus::zvariant::Value<'static>> {
        let state = self.state.lock().await;
        let mut metadata: HashMap<&'static str, zbus::zvariant::Value<'static>> = HashMap::new();

        let trackid = if state.playing {
            "/org/mpris/MediaPlayer2/Track1"
        } else {
            "/org/mpris/MediaPlayer2/NoTrack"
        };
        metadata.insert(
            "mpris:trackid",
            zbus::zvariant::Value::new(trackid.to_owned()),
        );

        let title = if state.track_title.is_empty() {
            state.station_name.clone()
        } else {
            state.track_title.clone()
        };
        if !title.is_empty() {
            metadata.insert("xesam:title", zbus::zvariant::Value::new(title));
        }

        if let Some(ref artist) = state.track_artist {
            metadata.insert(
                "xesam:artist",
                zbus::zvariant::Value::new(vec![artist.clone()]),
            );
        }

        if !state.url.is_empty() {
            metadata.insert("xesam:url", zbus::zvariant::Value::new(state.url.clone()));
        }

        metadata.insert("mpris:length", zbus::zvariant::Value::new(0i64));

        if let Some(ref icy) = state.icy_metadata {
            if let Some(ref genre) = icy.genre {
                metadata.insert("xesam:genre", zbus::zvariant::Value::new(genre.clone()));
            }
        }

        metadata
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
    fn can_play(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_pause(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_seek(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn can_control(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn shuffle(&self) -> bool {
        false
    }

    async fn next(&self) -> Result<()> {
        if let Some(cb) = self.next_callback.lock().await.as_ref() {
            cb();
        }
        Ok(())
    }

    async fn previous(&self) -> Result<()> {
        if let Some(cb) = self.prev_callback.lock().await.as_ref() {
            cb();
        }
        Ok(())
    }

    async fn pause(&self) -> Result<()> {
        if let Some(cb) = self.stop_callback.lock().await.as_ref() {
            cb();
        }
        Ok(())
    }

    async fn play_pause(&self) -> Result<()> {
        let playing = self.state.lock().await.playing;
        if playing {
            if let Some(cb) = self.stop_callback.lock().await.as_ref() {
                cb();
            }
        } else {
            let url = self.state.lock().await.url.clone();
            if let Some(cb) = self.play_callback.lock().await.as_ref() {
                cb(url);
            }
        }
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        if let Some(cb) = self.stop_callback.lock().await.as_ref() {
            cb();
        }
        Ok(())
    }

    async fn play(&self) -> Result<()> {
        let state = self.state.lock().await;
        let url = state.url.clone();
        drop(state);
        if let Some(cb) = self.play_callback.lock().await.as_ref() {
            cb(url);
        }
        Ok(())
    }

    async fn seek(&self, _offset: i64) -> Result<()> {
        Err(zbus::fdo::Error::NotSupported(
            "Seek not supported for radio streams".into(),
        ))
    }

    async fn set_position(&self, _track_id: &str, _position: i64) -> Result<()> {
        Err(zbus::fdo::Error::NotSupported(
            "SetPosition not supported for radio streams".into(),
        ))
    }

    async fn open_uri(&self, uri: &str) -> Result<()> {
        if let Some(cb) = self.play_callback.lock().await.as_ref() {
            cb(uri.to_string());
        }
        Ok(())
    }
}

pub struct MprisServer {
    state: Arc<Mutex<MprisState>>,
    connection: Option<Connection>,
    quit_callback: VoidCallback,
    play_callback: StringCallback,
    stop_callback: VoidCallback,
    volume_callback: FloatCallback,
    next_callback: VoidCallback,
    prev_callback: VoidCallback,
}

impl MprisServer {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(MprisState::default())),
            connection: None,
            quit_callback: Arc::new(Mutex::new(None)),
            play_callback: Arc::new(Mutex::new(None)),
            stop_callback: Arc::new(Mutex::new(None)),
            volume_callback: Arc::new(Mutex::new(None)),
            next_callback: Arc::new(Mutex::new(None)),
            prev_callback: Arc::new(Mutex::new(None)),
        }
    }

    pub fn state(&self) -> Arc<Mutex<MprisState>> {
        self.state.clone()
    }

    pub fn on_quit<F: Fn() + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.quit_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub fn on_play<F: Fn(String) + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.play_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub fn on_stop<F: Fn() + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.stop_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub fn on_volume_change<F: Fn(f64) + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.volume_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub fn on_next<F: Fn() + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.next_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub fn on_previous<F: Fn() + Send + Sync + 'static>(&self, callback: F) {
        if let Ok(mut cb) = self.prev_callback.try_lock() {
            *cb = Some(Box::new(callback));
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        let player = MediaPlayer2Player {
            state: self.state.clone(),
            play_callback: self.play_callback.clone(),
            stop_callback: self.stop_callback.clone(),
            volume_callback: self.volume_callback.clone(),
            next_callback: self.next_callback.clone(),
            prev_callback: self.prev_callback.clone(),
        };

        let root = MediaPlayer2 {
            quit_callback: self.quit_callback.clone(),
        };

        let connection = Connection::session().await?;
        connection
            .object_server()
            .at("/org/mpris/MediaPlayer2", root)
            .await?;
        connection
            .object_server()
            .at("/org/mpris/MediaPlayer2", player)
            .await?;
        connection
            .request_name("org.mpris.MediaPlayer2.radioboat")
            .await?;

        self.connection = Some(connection);
        Ok(())
    }

    pub async fn emit_properties_changed(&self) {
        if let Some(ref conn) = self.connection {
            let iface_ref = conn
                .object_server()
                .interface::<_, MediaPlayer2Player>("/org/mpris/MediaPlayer2")
                .await;

            if let Ok(iface_ref) = iface_ref {
                let state = self.state.lock().await;
                let mut metadata: HashMap<&str, zbus::zvariant::Value<'_>> = HashMap::new();

                let trackid = if state.playing {
                    "/org/mpris/MediaPlayer2/Track1"
                } else {
                    "/org/mpris/MediaPlayer2/NoTrack"
                };
                metadata.insert(
                    "mpris:trackid",
                    zbus::zvariant::Value::new(trackid.to_owned()),
                );

                let title = if state.track_title.is_empty() {
                    state.station_name.clone()
                } else {
                    state.track_title.clone()
                };
                if !title.is_empty() {
                    metadata.insert("xesam:title", zbus::zvariant::Value::new(title));
                }

                if let Some(ref artist) = state.track_artist {
                    metadata.insert(
                        "xesam:artist",
                        zbus::zvariant::Value::new(vec![artist.clone()]),
                    );
                }

                if !state.url.is_empty() {
                    metadata.insert("xesam:url", zbus::zvariant::Value::new(state.url.clone()));
                }

                metadata.insert("mpris:length", zbus::zvariant::Value::new(0i64));

                if let Some(ref icy) = state.icy_metadata {
                    if let Some(ref genre) = icy.genre {
                        metadata.insert("xesam:genre", zbus::zvariant::Value::new(genre.clone()));
                    }
                }

                let playback_status = if state.playing {
                    "Playing".to_string()
                } else {
                    "Stopped".to_string()
                };

                let mut changed: HashMap<&str, zbus::zvariant::Value<'_>> = HashMap::new();
                changed.insert("Metadata", zbus::zvariant::Value::new(metadata));
                changed.insert(
                    "PlaybackStatus",
                    zbus::zvariant::Value::new(playback_status),
                );

                drop(state);

                let emitter = iface_ref.signal_emitter();
                let _ = Properties::properties_changed(
                    emitter,
                    "org.mpris.MediaPlayer2.Player".try_into().unwrap(),
                    changed,
                    Cow::Borrowed(&[]),
                )
                .await;
            }
        }
    }
}

impl Default for MprisServer {
    fn default() -> Self {
        Self::new()
    }
}
