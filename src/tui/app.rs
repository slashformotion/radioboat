use std::time::{Duration, Instant};

use crate::config::{fetch_remote_stations, merge_stations, Import, Station};
use crate::icy;
use crate::player::{MpvPlayer, PlayerState};
use crate::tui::ui;

#[cfg(target_os = "linux")]
use crate::mpris::{MprisServer, MprisState};

#[cfg(target_os = "macos")]
use crate::macos::{MacOsMediaCenter, MacOsMediaState};

pub struct App {
    local_stations: Vec<Station>,
    stations: Vec<Station>,
    imports: Vec<Import>,
    cursor: usize,
    playing_index: Option<usize>,
    player: MpvPlayer,
    state: std::sync::Arc<tokio::sync::Mutex<PlayerState>>,
    messages: Vec<Message>,
    show_help: bool,
    size: ratatui::layout::Size,
    refreshing: bool,
    clipboard: Option<arboard::Clipboard>,
    #[cfg(target_os = "linux")]
    mpris_state: Option<std::sync::Arc<tokio::sync::Mutex<MprisState>>>,
    #[cfg(target_os = "linux")]
    mpris_server: Option<std::sync::Arc<tokio::sync::Mutex<MprisServer>>>,
    #[cfg(target_os = "macos")]
    macos_state: Option<std::sync::Arc<tokio::sync::Mutex<MacOsMediaState>>>,
    #[cfg(target_os = "macos")]
    macos_center: Option<std::sync::Arc<tokio::sync::Mutex<MacOsMediaCenter>>>,
}

#[derive(Debug)]
pub struct Message {
    content: String,
    is_error: bool,
    expires: Instant,
}

impl Message {
    pub fn info(msg: String) -> Self {
        Self {
            content: msg,
            is_error: false,
            expires: Instant::now() + Duration::from_secs(5),
        }
    }

    pub fn error(err: String) -> Self {
        Self {
            content: err,
            is_error: true,
            expires: Instant::now() + Duration::from_secs(5),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub const fn is_error(&self) -> bool {
        self.is_error
    }
}

impl App {
    pub fn new(
        local_stations: Vec<Station>,
        remote_stations: Vec<Station>,
        imports: Vec<Import>,
        player: MpvPlayer,
        import_errors: Vec<String>,
        #[cfg(target_os = "linux")] mpris_state: std::sync::Arc<tokio::sync::Mutex<MprisState>>,
    ) -> Self {
        let state = player.state().clone();
        let stations = merge_stations(local_stations.clone(), remote_stations);
        let messages = import_errors.into_iter().map(Message::error).collect();

        Self {
            local_stations,
            stations,
            imports,
            cursor: 0,
            playing_index: None,
            player,
            state,
            messages,
            show_help: false,
            size: ratatui::layout::Size::default(),
            refreshing: false,
            clipboard: arboard::Clipboard::new().ok(),
            #[cfg(target_os = "linux")]
            mpris_state: Some(mpris_state),
            #[cfg(target_os = "linux")]
            mpris_server: None,
            #[cfg(target_os = "macos")]
            macos_state: None,
            #[cfg(target_os = "macos")]
            macos_center: None,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn set_mpris(
        &mut self,
        state: std::sync::Arc<tokio::sync::Mutex<MprisState>>,
        server: std::sync::Arc<tokio::sync::Mutex<MprisServer>>,
    ) {
        self.mpris_state = Some(state);
        self.mpris_server = Some(server);
    }

    #[cfg(target_os = "macos")]
    pub fn set_macos(
        &mut self,
        state: std::sync::Arc<tokio::sync::Mutex<MacOsMediaState>>,
        center: std::sync::Arc<tokio::sync::Mutex<MacOsMediaCenter>>,
    ) {
        self.macos_state = Some(state);
        self.macos_center = Some(center);
    }

    pub const fn resize(&mut self, size: ratatui::layout::Size) {
        self.size = size;
    }

    pub async fn tick(&mut self) {
        self.messages.retain(|m| Instant::now() < m.expires);
        #[cfg(target_os = "linux")]
        self.sync_mpris_track().await;
        #[cfg(target_os = "macos")]
        self.sync_macos_track().await;
    }

    #[allow(clippy::too_many_lines)]
    pub async fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> anyhow::Result<bool> {
        use crossterm::event::KeyCode;

        if self.show_help {
            match key.code {
                KeyCode::Char('h') | KeyCode::Esc => self.show_help = false,
                _ => {}
            }
            return Ok(true);
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.player.close().await.ok();
                return Ok(false);
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.cursor < self.stations.len() - 1 {
                    self.cursor += 1;
                } else {
                    self.cursor = 0;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                } else {
                    self.cursor = self.stations.len() - 1;
                }
            }
            KeyCode::Char('h') | KeyCode::Left => {
                let page_height = self.page_height();
                if self.cursor >= page_height {
                    self.cursor -= page_height;
                } else {
                    self.cursor = 0;
                }
            }
            KeyCode::Char('l') | KeyCode::Right => {
                let page_height = self.page_height();
                self.cursor = (self.cursor + page_height).min(self.stations.len() - 1);
            }
            KeyCode::Char('m') => {
                if let Err(e) = self.player.toggle_mute().await {
                    self.messages
                        .push(Message::error(format!("Mute failed: {e}")));
                }
                #[cfg(target_os = "linux")]
                self.sync_mpris_mute().await;
            }
            KeyCode::Enter => {
                self.play_current_station().await?;
            }
            KeyCode::Char('*' | '+') => {
                if let Err(e) = self.player.volume_up().await {
                    self.messages
                        .push(Message::error(format!("Volume up failed: {e}")));
                }
                #[cfg(target_os = "linux")]
                self.sync_mpris_volume().await;
            }
            KeyCode::Char('/' | '-') => {
                if let Err(e) = self.player.volume_down().await {
                    self.messages
                        .push(Message::error(format!("Volume down failed: {e}")));
                }
                #[cfg(target_os = "linux")]
                self.sync_mpris_volume().await;
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Char('y') => {
                let player_state = self.state.lock().await;
                let track = player_state.current_track.clone();
                drop(player_state);

                if track.is_empty() {
                    self.messages
                        .push(Message::error("No song currently playing".to_string()));
                } else if let Some(ref mut clipboard) = self.clipboard {
                    match clipboard.set_text(&track) {
                        Ok(()) => self
                            .messages
                            .push(Message::info(format!("Copied: {track}"))),
                        Err(e) => self
                            .messages
                            .push(Message::error(format!("Copy failed: {e}"))),
                    }
                } else {
                    self.messages
                        .push(Message::error("Clipboard not available".to_string()));
                }
            }
            KeyCode::Char('r') if !self.imports.is_empty() => {
                self.refreshing = true;
                let imports = self.imports.clone();
                let (remote_stations, errors) = fetch_remote_stations(&imports).await;
                self.stations = merge_stations(self.local_stations.clone(), remote_stations);
                self.refreshing = false;

                if errors.is_empty() {
                    self.messages
                        .push(Message::info("Remote station lists refreshed".to_string()));
                } else {
                    for err in errors {
                        self.messages.push(Message::error(err));
                    }
                }

                if let Some(idx) = self.playing_index {
                    if idx >= self.stations.len() {
                        self.playing_index = None;
                    }
                }
                if self.cursor >= self.stations.len() && !self.stations.is_empty() {
                    self.cursor = self.stations.len() - 1;
                }
            }
            _ => {}
        }

        Ok(true)
    }

    fn page_height(&self) -> usize {
        (self.size.height as usize).saturating_sub(6).max(1)
    }

    pub fn draw_in(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        ui::draw(frame, self, area);
    }

    pub fn stations(&self) -> &[Station] {
        &self.stations
    }

    pub const fn cursor(&self) -> usize {
        self.cursor
    }

    pub const fn playing_index(&self) -> Option<usize> {
        self.playing_index
    }

    pub const fn state(&self) -> &std::sync::Arc<tokio::sync::Mutex<PlayerState>> {
        &self.state
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub const fn show_help(&self) -> bool {
        self.show_help
    }

    pub const fn has_imports(&self) -> bool {
        !self.imports.is_empty()
    }

    pub const fn is_refreshing(&self) -> bool {
        self.refreshing
    }

    #[cfg(target_os = "linux")]
    pub async fn quit(&self) -> anyhow::Result<()> {
        self.player.close().await.ok();
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub async fn play_url(&mut self, url: &str) -> anyhow::Result<()> {
        if let Some(idx) = self.stations.iter().position(|s| s.url == url) {
            self.cursor = idx;
            self.play_current_station().await?;
        } else {
            self.player.play(url).await?;
            self.playing_index = None;
            #[cfg(target_os = "linux")]
            {
                if let Some(ref mpris) = self.mpris_state {
                    let mut state = mpris.lock().await;
                    state.playing = true;
                    state.url = url.to_string();
                    state.station_name.clear();
                    state.track_title.clear();
                }
            }
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub async fn stop(&mut self) -> anyhow::Result<()> {
        self.player.stop().await?;
        self.playing_index = None;
        #[cfg(target_os = "linux")]
        {
            if let Some(ref mpris) = self.mpris_state {
                mpris.lock().await.playing = false;
            }
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub async fn next_station(&mut self) -> anyhow::Result<()> {
        if self.stations.is_empty() {
            return Ok(());
        }
        self.cursor = if self.cursor >= self.stations.len() - 1 {
            0
        } else {
            self.cursor + 1
        };
        self.play_current_station().await
    }

    #[cfg(target_os = "linux")]
    pub async fn previous_station(&mut self) -> anyhow::Result<()> {
        if self.stations.is_empty() {
            return Ok(());
        }
        self.cursor = if self.cursor == 0 {
            self.stations.len() - 1
        } else {
            self.cursor - 1
        };
        self.play_current_station().await
    }

    #[cfg(target_os = "linux")]
    pub async fn set_volume(&self, volume: i64) -> anyhow::Result<()> {
        let volume = volume.clamp(0, 110);
        self.player.set_volume(volume).await?;
        #[cfg(target_os = "linux")]
        self.sync_mpris_volume().await;
        Ok(())
    }

    async fn play_current_station(&mut self) -> anyhow::Result<()> {
        let station = &self.stations[self.cursor];
        self.player.play(&station.url).await?;
        self.playing_index = Some(self.cursor);

        let icy_metadata = icy::fetch_icy_metadata(&station.url).await;
        self.state.lock().await.icy_metadata = icy_metadata.clone();

        #[cfg(target_os = "linux")]
        {
            if let Some(ref mpris) = self.mpris_state {
                let mut state = mpris.lock().await;
                state.playing = true;
                state.url.clone_from(&station.url);
                state.station_name.clone_from(&station.name);
                state.track_title.clear();
                state.icy_metadata = icy_metadata;
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(ref macos) = self.macos_state {
                let mut state = macos.lock().await;
                state.playing = true;
                state.url = station.url.clone();
                state.station_name = station.name.clone();
                state.track_title.clear();
                state.icy_metadata = icy_metadata;
            }
            if let Some(ref center) = self.macos_center {
                center.lock().await.update_now_playing();
            }
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[allow(clippy::cast_precision_loss)]
    async fn sync_mpris_volume(&self) {
        if let Some(ref mpris) = self.mpris_state {
            let player_state = self.state.lock().await;
            mpris.lock().await.volume = player_state.volume as f64;
        }
    }

    #[cfg(target_os = "linux")]
    async fn sync_mpris_mute(&self) {
        if let Some(ref mpris) = self.mpris_state {
            let player_state = self.state.lock().await;
            mpris.lock().await.muted = player_state.muted;
        }
    }

    #[cfg(target_os = "linux")]
    async fn sync_mpris_track(&self) {
        if let Some(ref mpris) = self.mpris_state {
            let player_state = self.state.lock().await;
            let mut mpris_state = mpris.lock().await;
            let new_title = player_state.track_title.clone().unwrap_or_default();
            let track_changed = mpris_state.track_title != new_title;
            let artist_changed = mpris_state.track_artist != player_state.track_artist;
            let icy_changed = mpris_state.icy_metadata != player_state.icy_metadata;
            let bitrate_changed = mpris_state.audio_bitrate != player_state.audio_bitrate;

            if track_changed || artist_changed || icy_changed || bitrate_changed {
                mpris_state.track_title = new_title;
                mpris_state
                    .track_artist
                    .clone_from(&player_state.track_artist);
                mpris_state
                    .icy_metadata
                    .clone_from(&player_state.icy_metadata);
                mpris_state.audio_bitrate = player_state.audio_bitrate;
                drop(player_state);
                drop(mpris_state);
                if let Some(ref server) = self.mpris_server {
                    server.lock().await.emit_properties_changed().await;
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    async fn sync_macos_track(&self) {
        if let Some(ref macos) = self.macos_state {
            let player_state = self.state.lock().await;
            let mut macos_state = macos.lock().await;
            let new_title = player_state.track_title.clone().unwrap_or_default();
            let track_changed = macos_state.track_title != new_title;
            let artist_changed = macos_state.track_artist != player_state.track_artist;
            let icy_changed = macos_state.icy_metadata != player_state.icy_metadata;

            if track_changed || artist_changed || icy_changed {
                macos_state.track_title = new_title;
                macos_state.track_artist = player_state.track_artist.clone();
                macos_state.icy_metadata = player_state.icy_metadata.clone();
                drop(macos_state);
                if let Some(ref center) = self.macos_center {
                    center.lock().await.update_now_playing();
                }
            }
        }
    }
}
