```
██████   █████  ██████  ██  ██████  ██████   ██████   █████  ████████ 
██   ██ ██   ██ ██   ██ ██ ██    ██ ██   ██ ██    ██ ██   ██    ██    
██████  ███████ ██   ██ ██ ██    ██ ██████  ██    ██ ███████    ██    
██   ██ ██   ██ ██   ██ ██ ██    ██ ██   ██ ██    ██ ██   ██    ██    
██   ██ ██   ██ ██████  ██  ██████  ██████   ██████  ██   ██    ██    
```

![Contributors](https://img.shields.io/github/contributors/slashformotion/radioboat)
![Forks](https://img.shields.io/github/forks/slashformotion/radioboat)
![Stars](https://img.shields.io/github/stars/slashformotion/radioboat)
![Licence](https://img.shields.io/github/license/slashformotion/radioboat)
![Issues](https://img.shields.io/github/issues/slashformotion/radioboat)

**Radioboat is a terminal web radio client, built with simplicity in mind**

## Features

- Play web radio stations via MPV
- TOML-based configuration
- Remote station list imports (HTTP/HTTPS or local files)
- Full-screen or compact UI mode
- Async, non-blocking interface

## Installation

### From source (Nix)

```bash
nix develop --command cargo install --path .
```

### From source (Cargo)

```bash
cargo install --path .
```

Requirements: [Rust](https://rustup.rs/) 1.70+, [mpv](https://mpv.io/)

## Usage

```bash
# Run with default config (~/.config/radioboat/radioboat.toml)
radioboat

# Use custom config
radioboat --config ~/my-config.toml

# Small window mode
radioboat --ui-size small

# Edit config in $EDITOR
radioboat config-edit
```

## Configuration

Config file: `~/.config/radioboat/radioboat.toml`

```toml
volume = 80
muted = false

# Optional: import remote station lists
[[imports]]
name = "My Remote Stations"
url = "https://example.com/stations.toml"

[[imports]]
name = "Local Backup"
url = "~/.config/radioboat/extra.toml"

[[stations]]
name = "Jazz FM"
url = "https://stream.example.com/jazz"

[[stations]]
name = "Deep House"
url = "http://channels.dinamo.fm/deep-mp3"
```

Remote station lists contain only `[[stations]]` entries. See [radioboat.toml](radioboat.toml) and [remote.toml](remote.toml) for examples.

## Keybindings

| Key | Action |
|-----|--------|
| `↑/k` | Move up |
| `↓/j` | Move down |
| `←/h` | Page left |
| `→/l` | Page right |
| `Enter` | Play station |
| `m` | Toggle mute |
| `*`/`+` | Volume up |
| `-`/`/` | Volume down |
| `r` | Refresh remote lists |
| `?` | Show help |
| `q/Esc` | Quit |

> Stations show their source in brackets: `[local]` or `[import name]` when imports are configured

## Documentation

- [docs/README.md](docs/README.md) - Overview
- [docs/config.md](docs/config.md) - Configuration details
- [docs/architecture.md](docs/architecture.md) - Technical architecture

## Dependencies

- [mpv](https://mpv.io/) - Audio playback
- [ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI
- [tokio](https://tokio.rs/) - Async runtime

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## License

Apache 2.0 - See [LICENCE](LICENCE)
