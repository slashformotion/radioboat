use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum MprisCommand {
    Quit,
    Play(String),
    Stop,
    Next,
    Previous,
    SetVolume(f64),
}

pub enum Event {
    Key(KeyEvent),
    Tick,
    Resize(ratatui::layout::Size),
    #[cfg(target_os = "linux")]
    Mpris(MprisCommand),
}

pub struct EventHandler {
    event_rx: tokio::sync::mpsc::Receiver<Event>,
    event_tx: tokio::sync::mpsc::Sender<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(100);
        let tx = event_tx.clone();

        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(tick_rate);

            loop {
                let tick_tick = tick.tick().boxed();
                let crossterm_event = reader.next().boxed();

                tokio::select! {
                    _ = tick_tick => {
                        if event_tx.send(Event::Tick).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(event)) = crossterm_event => {
                        match event {
                            CrosstermEvent::Key(key) => {
                                if event_tx.send(Event::Key(key)).await.is_err() {
                                    break;
                                }
                            }
                            CrosstermEvent::Resize(cols, rows) => {
                                if event_tx.send(Event::Resize(ratatui::layout::Size::new(cols, rows))).await.is_err() {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        });

        Self {
            event_rx,
            event_tx: tx,
        }
    }

    pub fn sender(&self) -> tokio::sync::mpsc::Sender<Event> {
        self.event_tx.clone()
    }

    pub async fn next(&mut self) -> Event {
        self.event_rx.recv().await.expect("event channel closed")
    }
}
