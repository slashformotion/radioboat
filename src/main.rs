mod config;
mod icy;
mod player;
mod tui;

#[cfg(target_os = "linux")]
mod mpris;

#[cfg(target_os = "macos")]
mod macos;

use std::io::stdout;
use std::path::PathBuf;
use std::time::Duration;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use config::{fetch_remote_stations, load_config};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use player::MpvPlayer;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::app::App;
#[cfg(target_os = "linux")]
use tui::event::MprisCommand;
use tui::event::{Event, EventHandler};

const DEFAULT_CONFIG_PATH: &str = "~/.config/radioboat/radioboat.toml";

#[derive(Parser, Debug)]
#[command(name = "radioboat")]
#[command(about = "A terminal web radio client", long_about = None)]
#[command(version)]
struct Args {
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
    config: String,

    #[arg(long, default_value = "full")]
    ui_size: UiSize,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum UiSize {
    Full,
    Small,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "config-edit", about = "Open config file in $EDITOR")]
    ConfigEdit {
        #[arg(short, long, default_value = DEFAULT_CONFIG_PATH)]
        config: String,
    },

    #[command(about = "Generate shell completions")]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
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
        Some(Commands::Completion { shell }) => {
            generate(shell, &mut Args::command(), "radioboat", &mut stdout());
            return Ok(());
        }
        None => {}
    }

    let config = load_config(&args.config)?;

    let local_stations = config.stations;
    let imports = config.imports;

    if local_stations.is_empty() && imports.is_empty() {
        eprintln!("No stations found in {}", args.config);
        std::process::exit(1);
    }

    let (remote_stations, import_errors) = fetch_remote_stations(&imports).await;

    if local_stations.is_empty() && remote_stations.is_empty() {
        eprintln!("No stations found (local or remote)");
        std::process::exit(1);
    }

    let player = MpvPlayer::new().await?;
    player.set_volume(config.volume).await?;
    if config.muted {
        player.toggle_mute().await?;
    }

    #[cfg(target_os = "linux")]
    let mpris_state = {
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let state = Arc::new(Mutex::new(mpris::MprisState::default()));
        state.lock().await.volume = config.volume as f64;
        state.lock().await.muted = config.muted;
        state
    };

    let mut app = App::new(
        local_stations,
        remote_stations,
        imports,
        player,
        import_errors,
        #[cfg(target_os = "linux")]
        mpris_state.clone(),
    );

    let mut terminal = setup_terminal(args.ui_size)?;

    let event_handler = EventHandler::new(Duration::from_millis(100));

    #[cfg(target_os = "linux")]
    let _mpris_server = {
        let sender = event_handler.sender();
        let mut mpris_server = mpris::MprisServer::new();
        let mpris_state_clone = mpris_server.state();

        {
            let sender = sender.clone();
            mpris_server.on_quit(move || {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(Event::Mpris(MprisCommand::Quit)).await;
                });
            });
        }

        {
            let sender = sender.clone();
            mpris_server.on_play(move |url| {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(Event::Mpris(MprisCommand::Play(url))).await;
                });
            });
        }

        {
            let sender = sender.clone();
            mpris_server.on_stop(move || {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(Event::Mpris(MprisCommand::Stop)).await;
                });
            });
        }

        {
            let sender = sender.clone();
            mpris_server.on_next(move || {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(Event::Mpris(MprisCommand::Next)).await;
                });
            });
        }

        {
            let sender = sender.clone();
            mpris_server.on_previous(move || {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender.send(Event::Mpris(MprisCommand::Previous)).await;
                });
            });
        }

        {
            let sender = sender.clone();
            mpris_server.on_volume_change(move |vol| {
                let sender = sender.clone();
                tokio::spawn(async move {
                    let _ = sender
                        .send(Event::Mpris(MprisCommand::SetVolume(vol)))
                        .await;
                });
            });
        }

        if let Err(e) = mpris_server.start().await {
            eprintln!("Warning: Failed to start MPRIS server: {e}");
        }

        let mpris_server = std::sync::Arc::new(tokio::sync::Mutex::new(mpris_server));
        app.set_mpris(mpris_state_clone, mpris_server.clone());
        Some(mpris_server)
    };
    #[cfg(not(target_os = "linux"))]
    let _mpris_server: Option<()> = None;

    #[cfg(target_os = "macos")]
    let _macos_center = {
        let mut macos_center = macos::MacOsMediaCenter::new();
        let macos_state = macos_center.state();

        if let Err(e) = macos_center.start() {
            eprintln!("Warning: Failed to start macOS media center: {e}");
        }

        app.set_macos(
            macos_state,
            std::sync::Arc::new(tokio::sync::Mutex::new(macos_center)),
        );
        Some(())
    };
    #[cfg(not(target_os = "macos"))]
    let _macos_center: Option<()> = None;

    let res = run_app(&mut terminal, &mut app, event_handler, args.ui_size).await;

    drop(_mpris_server);
    let _ = _macos_center;
    restore_terminal(args.ui_size)?;

    if let Err(e) = res {
        eprintln!("Error: {e}");
    }

    println!("Exiting...");

    Ok(())
}

fn setup_terminal(ui_size: UiSize) -> anyhow::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    if matches!(ui_size, UiSize::Small) {
        execute!(
            stdout,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )?;
    } else {
        execute!(stdout, EnterAlternateScreen)?;
    }
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(ui_size: UiSize) -> anyhow::Result<()> {
    if matches!(ui_size, UiSize::Full) {
        execute!(stdout(), LeaveAlternateScreen)?;
    }
    disable_raw_mode()?;
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    mut event_handler: EventHandler,
    ui_size: UiSize,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| {
            let area = if matches!(ui_size, UiSize::Small) {
                let size = f.area();
                let height = (size.height / 2).clamp(10, 20);
                ratatui::layout::Rect::new(size.x, size.y, size.width, height)
            } else {
                f.area()
            };
            app.draw_in(f, area);
        })?;

        match event_handler.next().await {
            Event::Key(key) => {
                if !app.handle_key(key).await? {
                    return Ok(());
                }
            }
            Event::Tick => {
                app.tick().await;
            }
            Event::Resize(size) => {
                app.resize(size);
            }
            #[cfg(target_os = "linux")]
            Event::Mpris(cmd) => match cmd {
                MprisCommand::Quit => {
                    app.quit().await?;
                    return Ok(());
                }
                MprisCommand::Play(url) => {
                    app.play_url(&url).await?;
                }
                MprisCommand::Stop => {
                    app.stop().await?;
                }
                MprisCommand::Next => {
                    app.next_station().await?;
                }
                MprisCommand::Previous => {
                    app.previous_station().await?;
                }
                MprisCommand::SetVolume(vol) => {
                    app.set_volume(vol as i64).await?;
                }
            },
        }
    }
}

const DEFAULT_CONFIG_TEMPLATE: &str = r#"volume = 80
muted = false

# Optional: import remote station lists
# [[imports]]
# name = "My Remote Stations"
# url = "https://example.com/stations.toml"

[[stations]]
name = "Example Station"
url = "https://example.com/stream"

[[stations]]
name = "Another Station"
url = "https://example.org/stream"
"#;
