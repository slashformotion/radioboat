use std::time::{Duration, Instant};

use crate::config::{fetch_remote_stations, merge_stations, Station};
use crate::player::{MpvPlayer, PlayerState};
use crate::tui::ui;

pub struct App {
    local_stations: Vec<Station>,
    stations: Vec<Station>,
    imports: Vec<String>,
    cursor: usize,
    playing_index: Option<usize>,
    player: MpvPlayer,
    state: std::sync::Arc<tokio::sync::Mutex<PlayerState>>,
    messages: Vec<Message>,
    show_help: bool,
    size: ratatui::layout::Size,
    refreshing: bool,
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

    pub fn is_error(&self) -> bool {
        self.is_error
    }
}

impl App {
    pub fn new(
        local_stations: Vec<Station>,
        remote_stations: Vec<Station>,
        imports: Vec<String>,
        player: MpvPlayer,
        import_errors: Vec<String>,
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
        }
    }

    pub fn resize(&mut self, size: ratatui::layout::Size) {
        self.size = size;
    }

    pub fn tick(&mut self) {
        self.messages.retain(|m| Instant::now() < m.expires);
    }

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
                    self.messages.push(Message::error(format!("Mute failed: {}", e)));
                }
            }
            KeyCode::Enter => {
                let station = &self.stations[self.cursor];
                match self.player.play(&station.url).await {
                    Ok(()) => {
                        self.playing_index = Some(self.cursor);
                    }
                    Err(e) => {
                        self.messages.push(Message::error(format!("Play failed: {}", e)));
                    }
                }
            }
            KeyCode::Char('*') | KeyCode::Char('+') => {
                if let Err(e) = self.player.volume_up().await {
                    self.messages.push(Message::error(format!("Volume up failed: {}", e)));
                }
            }
            KeyCode::Char('/') | KeyCode::Char('-') => {
                if let Err(e) = self.player.volume_down().await {
                    self.messages.push(Message::error(format!("Volume down failed: {}", e)));
                }
            }
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Char('r') if !self.imports.is_empty() => {
                self.refreshing = true;
                let imports = self.imports.clone();
                let (remote_stations, errors) = fetch_remote_stations(&imports).await;
                self.stations = merge_stations(self.local_stations.clone(), remote_stations);
                self.refreshing = false;

                if errors.is_empty() {
                    self.messages.push(Message::info("Remote station lists refreshed".to_string()));
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

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn playing_index(&self) -> Option<usize> {
        self.playing_index
    }

    pub fn state(&self) -> &std::sync::Arc<tokio::sync::Mutex<PlayerState>> {
        &self.state
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn has_imports(&self) -> bool {
        !self.imports.is_empty()
    }

    pub fn is_refreshing(&self) -> bool {
        self.refreshing
    }
}
