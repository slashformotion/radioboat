package tui

import "github.com/charmbracelet/bubbles/key"

type KeyMap struct {
	Up         key.Binding
	Down       key.Binding
	ToggleMute key.Binding
	Play       key.Binding
	Quit       key.Binding
	VolumeUp   key.Binding
	VolumeDown key.Binding
}

var DefaultKeyMap = KeyMap{
	Up: key.NewBinding(
		key.WithKeys("k", "up"),        // actual keybindings
		key.WithHelp("↑/k", "move up"), // corresponding help text
	),
	Down: key.NewBinding(
		key.WithKeys("j", "down"),
		key.WithHelp("↓/j", "move down"),
	),
	ToggleMute: key.NewBinding(
		key.WithKeys("m"),
		key.WithHelp("m", "mute/unmute"),
	),
	Play: key.NewBinding(
		key.WithKeys("enter"),
		key.WithHelp("Enter", "Play station"),
	),
	VolumeUp: key.NewBinding(
		key.WithKeys("*"),
		key.WithHelp("*", "Increase volume"),
	),
	VolumeDown: key.NewBinding(
		key.WithKeys("/"),
		key.WithHelp("/", "Decrease volume"),
	),
	Quit: key.NewBinding(
		key.WithKeys("ctrl+c", "esc", "q"),
		key.WithHelp("Ctrl+C/Escape/q", "Quit"),
	),
}

// ShortHelp returns keybindings to be shown in the mini help view. It's part
// of the key.Map interface.
func (k KeyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.Quit, k.Up, k.Down, k.VolumeDown, k.VolumeUp, k.Play, k.ToggleMute}
}

// FullHelp returns keybindings for the expanded help view. It's part of the
// key.Map interface.
func (k KeyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{k.Up, k.Down, k.VolumeDown, k.VolumeUp, k.Play, k.ToggleMute}, // first column
		{k.Quit}, // second column
	}
}
