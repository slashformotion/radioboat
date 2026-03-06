# Architecture

## Components

```
src/
├── main.rs       # Entry point, CLI, terminal setup
├── config.rs     # Config parsing, remote fetch, station merge
├── player.rs     # MPV IPC via Unix socket
└── tui/
    ├── app.rs    # Application state, key handling
    ├── event.rs  # Async event loop
    └── ui.rs     # Ratatui rendering
```

## Data Flow

```
Config File → load_config() → App
                    ↓
            fetch_remote_stations() → merge_stations()
                    ↓
                  App.stations
                    ↓
User Input → handle_key() → MpvPlayer
                    ↓
             MPV (via socket)
```

## MPV Communication

- Uses Unix socket IPC (`--input-ipc-server`)
- JSON commands sent over socket
- Background task reads events for track metadata

## Async Model

- Tokio runtime
- Event channel for keyboard/tick/resize
- Player state behind `Arc<Mutex<_>>`
- UI never blocks on I/O
