# Configuration

Config location: `~/.config/radioboat/radioboat.toml`

## Example

```toml
volume = 80
muted = false

imports = [
    "https://example.com/stations.toml",
    "~/.config/radioboat/extra.toml"
]

[[stations]]
name = "My Station"
url = "https://stream.example.com/mp3"
```

## Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `volume` | int | 80 | Initial volume (0-110) |
| `muted` | bool | false | Start muted |
| `imports` | [string] | [] | Remote station list URLs/paths |
| `stations` | [[Station]] | [] | Local station list |

## Station Object

```toml
[[stations]]
name = "Station Name"  # Required
url = "https://..."    # Required
```

## Remote Station Lists

Remote TOML files contain only stations:

```toml
[[stations]]
name = "Remote Station"
url = "https://..."
```

### Import Behavior

- Fetched once at startup
- Press `r` to refresh
- Local stations override remote duplicates
- Merged alphabetically
- Errors shown in UI, app continues

### Sources

- HTTP/HTTPS URLs
- Local file paths (`~` expanded)
