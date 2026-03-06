mod config;
mod player;
mod tui;

use std::time::Duration;

use clap::Parser;
use config::{load_config, Station};
use player::MpvPlayer;
use tui::app::App;
use tui::event::{Event, EventHandler};

#[derive(Parser, Debug)]
#[command(name = "radioboat")]
#[command(about = "A terminal web radio client", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "~/.config/radioboat/radioboat.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = load_config(&args.config)?;

    if config.stations.is_empty() {
        eprintln!("No stations found in {}", args.config);
        std::process::exit(1);
    }

    let stations: Vec<Station> = config.stations;

    let player = MpvPlayer::new().await?;
    player.set_volume(config.volume).await?;
    if config.muted {
        player.toggle_mute().await?;
    }

    let mut app = App::new(stations, player);

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let event_handler = EventHandler::new(Duration::from_millis(100));

    let res = run_app(&mut terminal, &mut app, event_handler).await;

    ratatui::restore();

    if let Err(e) = res {
        eprintln!("Error: {e}");
    }

    println!("Exiting...");

    Ok(())
}

async fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
    mut event_handler: EventHandler,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| app.draw(f))?;

        match event_handler.next().await {
            Event::Key(key) => {
                if !app.handle_key(key).await? {
                    return Ok(());
                }
            }
            Event::Tick => {
                app.tick();
            }
            Event::Resize(size) => {
                app.resize(size);
            }
        }
    }
}
