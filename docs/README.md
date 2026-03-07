# Radioboat

A terminal web radio client built with Rust and ratatui.

## Features

- **Terminal UI** - Full-screen or compact mode
- **MPV Backend** - Streams via mpv player
- **Remote Imports** - Fetch station lists from URLs or local files
- **Async** - Non-blocking UI with tokio

## Installation

```bash
nix develop --command cargo install --path .
```

Requires: mpv, Rust 1.70+

## Usage

```bash
# Run with default config
radioboat

# Use custom config
radioboat --config ~/my-config.toml

# Small window mode
radioboat --ui-size small

# Edit config
radioboat config-edit
```

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

## Environment

- `RADIOBOAT_EDITOR` - Editor for config-edit (overrides `$EDITOR`)
- `EDITOR` - Fallback editor (defaults to `nano`)
