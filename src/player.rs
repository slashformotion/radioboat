use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

use crate::icy::IcyMetadata;

const USER_DATA_MEDIA_TITLE: u64 = 20_000_001;
const USER_DATA_AUDIO_BITRATE: u64 = 20_000_002;
const USER_DATA_AUDIO_PARAMS: u64 = 20_000_003;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct AudioParams {
    pub samplerate: Option<u32>,
    pub channel_count: Option<u32>,
    pub format: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub volume: i64,
    pub muted: bool,
    pub current_track: String,
    pub track_artist: Option<String>,
    pub track_title: Option<String>,
    pub icy_metadata: Option<IcyMetadata>,
    pub audio_bitrate: Option<u32>,
    #[allow(dead_code)]
    pub audio_params: Option<AudioParams>,
}

pub struct MpvPlayer {
    socket_path: PathBuf,
    state: Arc<Mutex<PlayerState>>,
    _child: Arc<Mutex<Option<Child>>>,
}

impl MpvPlayer {
    pub async fn new() -> anyhow::Result<Self> {
        let socket_path =
            std::env::temp_dir().join(format!("radioboat-mpv-{}.sock", std::process::id()));

        if socket_path.exists() {
            std::fs::remove_file(&socket_path)?;
        }

        let child = Command::new("mpv")
            .args([
                "--idle=yes",
                "--no-video",
                "--force-window=no",
                &format!("--input-ipc-server={}", socket_path.display()),
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let state = Arc::new(Mutex::new(PlayerState {
            volume: 80,
            muted: false,
            current_track: String::new(),
            track_artist: None,
            track_title: None,
            icy_metadata: None,
            audio_bitrate: None,
            audio_params: None,
        }));

        let player = Self {
            socket_path,
            state: state.clone(),
            _child: Arc::new(Mutex::new(Some(child))),
        };

        for _ in 0..50 {
            if player.socket_path.exists() {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        if !player.socket_path.exists() {
            anyhow::bail!("mpv socket not created");
        }

        let socket_path_clone = player.socket_path.clone();
        tokio::spawn(async move {
            loop {
                if let Ok(stream) = UnixStream::connect(&socket_path_clone).await {
                    let (reader, writer) = stream.into_split();
                    let writer = Arc::new(tokio::sync::Mutex::new(writer));

                    let mut w = writer.lock().await;

                    let cmd = serde_json::json!({
                        "command": ["observe_property", USER_DATA_MEDIA_TITLE, "media-title"]
                    });
                    let _ = w
                        .write_all((serde_json::to_string(&cmd).unwrap() + "\n").as_bytes())
                        .await;

                    let cmd = serde_json::json!({
                        "command": ["observe_property", USER_DATA_AUDIO_BITRATE, "audio-bitrate"]
                    });
                    let _ = w
                        .write_all((serde_json::to_string(&cmd).unwrap() + "\n").as_bytes())
                        .await;

                    let cmd = serde_json::json!({
                        "command": ["observe_property", USER_DATA_AUDIO_PARAMS, "audio-params"]
                    });
                    let _ = w
                        .write_all((serde_json::to_string(&cmd).unwrap() + "\n").as_bytes())
                        .await;

                    let _ = w.flush().await;
                    drop(w);

                    let reader = BufReader::new(reader);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                            if let Some(event) = json.get("event").and_then(|e| e.as_str()) {
                                if event == "property-change" {
                                    if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                                        match name {
                                            "media-title" => {
                                                if let Some(track) =
                                                    json.get("data").and_then(|d| d.as_str())
                                                {
                                                    let mut s = state.lock().await;
                                                    s.current_track = track.to_string();
                                                    if let Some((artist, title)) =
                                                        parse_artist_title(track)
                                                    {
                                                        s.track_artist = Some(artist);
                                                        s.track_title = Some(title);
                                                    } else {
                                                        s.track_artist = None;
                                                        s.track_title = Some(track.to_string());
                                                    }
                                                }
                                            }
                                            "audio-bitrate" => {
                                                if let Some(bitrate) =
                                                    json.get("data").and_then(|d| d.as_u64())
                                                {
                                                    let mut s = state.lock().await;
                                                    s.audio_bitrate = Some(bitrate as u32);
                                                }
                                            }
                                            "audio-params" => {
                                                if let Some(params) = json.get("data") {
                                                    let mut s = state.lock().await;
                                                    s.audio_params = Some(AudioParams {
                                                        samplerate: params
                                                            .get("samplerate")
                                                            .and_then(|v| v.as_u64())
                                                            .map(|v| v as u32),
                                                        channel_count: params
                                                            .get("channel-count")
                                                            .and_then(|v| v.as_u64())
                                                            .map(|v| v as u32),
                                                        format: params
                                                            .get("format")
                                                            .and_then(|v| v.as_str())
                                                            .map(String::from),
                                                    });
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            }
        });

        player.set_volume(80).await?;

        Ok(player)
    }

    async fn send_command(&self, args: &[&str]) -> anyhow::Result<()> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        let (_, mut writer) = stream.into_split();

        let cmd = serde_json::json!({
            "command": args
        });

        let line = serde_json::to_string(&cmd)? + "\n";
        writer.write_all(line.as_bytes()).await?;
        writer.flush().await?;

        Ok(())
    }

    pub async fn play(&self, url: &str) -> anyhow::Result<()> {
        self.send_command(&["loadfile", url, "replace"]).await
    }

    pub async fn toggle_mute(&self) -> anyhow::Result<()> {
        let muted = self.state.lock().await.muted;
        let new_muted = !muted;
        self.send_command(&["set_property", "mute", if new_muted { "yes" } else { "no" }])
            .await?;
        self.state.lock().await.muted = new_muted;
        Ok(())
    }

    pub async fn volume_up(&self) -> anyhow::Result<()> {
        let volume = self.state.lock().await.volume;
        let new_volume = (volume + 5).min(110);
        self.set_volume(new_volume).await
    }

    pub async fn volume_down(&self) -> anyhow::Result<()> {
        let volume = self.state.lock().await.volume;
        let new_volume = (volume - 5).max(0);
        self.set_volume(new_volume).await
    }

    pub async fn set_volume(&self, volume: i64) -> anyhow::Result<()> {
        self.send_command(&["set_property", "volume", &volume.to_string()])
            .await?;
        self.state.lock().await.volume = volume;
        Ok(())
    }

    pub const fn state(&self) -> &Arc<Mutex<PlayerState>> {
        &self.state
    }

    pub async fn close(&self) -> anyhow::Result<()> {
        let _ = self.send_command(&["quit"]).await;
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path)?;
        }
        Ok(())
    }
}

impl Drop for MpvPlayer {
    fn drop(&mut self) {
        if self.socket_path.exists() {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
}

fn parse_artist_title(track: &str) -> Option<(String, String)> {
    let separator = track.find(" - ")?;
    let artist = track[..separator].trim();
    let title = track[separator + 3..].trim();
    if artist.is_empty() || title.is_empty() {
        return None;
    }
    Some((artist.to_string(), title.to_string()))
}
