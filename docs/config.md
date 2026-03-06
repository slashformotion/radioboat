# Configuration

Config location: `~/.config/radioboat/radioboat.toml`

## Example

```toml
volume = 80
muted = false

[[imports]]
name = "My Remote Stations"
url = "https://example.com/stations.toml"

[[imports]]
name = "Local Backup"
url = "~/.config/radioboat/extra.toml"

[[stations]]
name = "My Station"
url = "https://stream.example.com/mp3"
```

## Fields

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `volume` | int | 80 | Initial volume (0-110) |
| `muted` | bool | false | Start muted |
| `imports` | [[Import]] | [] | Remote station lists |
| `stations` | [[Station]] | [] | Local station list |

## Station Object

```toml
[[stations]]
name = "Station Name"  # Required
url = "https://..."    # Required
```

## Import Object

```toml
[[imports]]
name = "List Name"     # Required - shown in error messages
url = "https://..."    # Required - URL or local path
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
- Errors shown with list name, app continues

### Sources

- HTTP/HTTPS URLs
- Local file paths (`~` expanded)
