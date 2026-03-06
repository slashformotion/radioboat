mod config;
mod player;
mod tui;

use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};
use config::{load_config, Station};
use player::MpvPlayer;
use tui::app::App;
use tui::event::{Event, EventHandler};

const DEFAULT_CONFIG_PATH: &str = "~/.config/radioboat/radioboat.toml";

#[derive(Parser, Debug)]
#[command(name = "radioboat")]
#[command(about = "A terminal web radio client", long_about = None)]
struct Args {
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
    config: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "config-edit", about = "Open config file in $EDITOR")]
    ConfigEdit {
        #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
        config: String,
    },
}

fn expand_tilde(path: &str) -> PathBuf {
    shellexpand::tilde(path).to_string().into()
}

fn get_editor() -> String {
    std::env::var("RADIOBOAT_EDITOR")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| "nano".to_string())
}

fn open_in_editor(path: &str) -> anyhow::Result<()> {
    let expanded_path = expand_tilde(path);
    
    let editor = get_editor();
    
    let status = std::process::Command::new(&editor)
        .arg(&expanded_path)
        .status()?;
    
    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::ConfigEdit { config }) => {
            let expanded_path = expand_tilde(&config);
            if !expanded_path.exists() {
                if let Some(parent) = expanded_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&expanded_path, DEFAULT_CONFIG_TEMPLATE)?;
            }
            open_in_editor(&config)?;
            return Ok(());
        }
        None => {}
    }

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

const DEFAULT_CONFIG_TEMPLATE: &str = r#"volume = 80
muted = false

[[stations]]
name = "Example Station"
url = "https://example.com/stream"

[[stations]]
name = "Another Station"
url = "https://example.org/stream"
"#;
