use std::time::{Duration, Instant};

use crate::config::Station;
use crate::player::{MpvPlayer, PlayerState};
use crate::tui::ui;

pub struct App {
    stations: Vec<Station>,
    cursor: usize,
    playing_index: Option<usize>,
    player: MpvPlayer,
    state: std::sync::Arc<tokio::sync::Mutex<PlayerState>>,
    messages: Vec<Message>,
    show_help: bool,
    size: ratatui::layout::Size,
}

#[derive(Debug)]
pub struct Message {
    content: String,
    is_error: bool,
    expires: Instant,
}

impl Message {
    fn new(content: String) -> Self {
        Self {
            content,
            is_error: false,
            expires: Instant::now() + Duration::from_secs(5),
        }
    }

    fn error(err: String) -> Self {
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
    pub fn new(stations: Vec<Station>, player: MpvPlayer) -> Self {
        let state = player.state().clone();
        Self {
            stations,
            cursor: 0,
            playing_index: None,
            player,
            state,
            messages: Vec::new(),
            show_help: false,
            size: ratatui::layout::Size::default(),
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
            _ => {}
        }

        Ok(true)
    }

    fn page_height(&self) -> usize {
        (self.size.height as usize).saturating_sub(6).max(1)
    }

    pub fn draw(&self, frame: &mut ratatui::Frame) {
        ui::draw(frame, self);
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
}
