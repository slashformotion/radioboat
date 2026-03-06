use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

const USER_DATA_MEDIA_TITLE: u64 = 20_000_001;

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub volume: i64,
    pub muted: bool,
    pub current_track: String,
}

pub struct MpvPlayer {
    socket_path: PathBuf,
    state: Arc<Mutex<PlayerState>>,
    _child: Arc<Mutex<Option<Child>>>,
}

impl MpvPlayer {
    pub async fn new() -> anyhow::Result<Self> {
        let socket_path = std::env::temp_dir().join(format!("radioboat-mpv-{}.sock", std::process::id()));

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
                    
                    let cmd = serde_json::json!({
                        "command": ["observe_property", USER_DATA_MEDIA_TITLE, "media-title"]
                    });
                    let cmd_str = serde_json::to_string(&cmd).unwrap() + "\n";
                    
                    let mut w = writer.lock().await;
                    let _ = w.write_all(cmd_str.as_bytes()).await;
                    let _ = w.flush().await;
                    drop(w);
                    
                    let reader = BufReader::new(reader);
                    let mut lines = reader.lines();
                    
                    while let Ok(Some(line)) = lines.next_line().await {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                            if let Some(event) = json.get("event").and_then(|e| e.as_str()) {
                                if event == "property-change" {
                                    if let Some(name) = json.get("name").and_then(|n| n.as_str()) {
                                        if name == "media-title" {
                                            if let Some(track) = json.get("data").and_then(|d| d.as_str()) {
                                                let mut s = state.lock().await;
                                                s.current_track = track.to_string();
                                            }
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
        self.send_command(&["set_property", "mute", if new_muted { "yes" } else { "no" }]).await?;
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
        self.send_command(&["set_property", "volume", &volume.to_string()]).await?;
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
