package tui

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import "github.com/charmbracelet/bubbles/key"

type KeyMap struct {
	Up         key.Binding
	Down       key.Binding
	Right      key.Binding
	Left       key.Binding
	ToggleMute key.Binding
	Play       key.Binding
	Quit       key.Binding
	VolumeUp   key.Binding
	VolumeDown key.Binding
	SaveTrack  key.Binding
	Help       key.Binding
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
	Right: key.NewBinding(
		key.WithKeys("l", "right"),
		key.WithHelp("→/l", "move right"),
	),
	Left: key.NewBinding(
		key.WithKeys("h", "left"),
		key.WithHelp("←/j", "move left"),
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
		key.WithKeys("*", "-"),
		key.WithHelp("*", "Increase volume"),
	),
	VolumeDown: key.NewBinding(
		key.WithKeys("/", "-"),
		key.WithHelp("/", "Decrease volume"),
	),
	Quit: key.NewBinding(
		key.WithKeys("ctrl+c", "esc", "q"),
		key.WithHelp("Ctrl+C/Escape/q", "Quit"),
	),
	SaveTrack: key.NewBinding(
		key.WithKeys("ctrl+s"),
		key.WithHelp("Ctrl+s", "Save current track name to track file"),
	),
	Help: key.NewBinding(
		key.WithKeys("h"),
		key.WithHelp("h", "Show All keybindings"),
	),
}

// ShortHelp returns keybindings to be shown in the mini help view. It's part
// of the key.Map interface.
func (k KeyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.Help, k.VolumeDown, k.VolumeUp, k.Play}
}

// FullHelp returns keybindings for the expanded help view. It's part of the
// key.Map interface.
func (k KeyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{k.Help, k.Play},
		{k.Up, k.Down},
		{k.Left, k.Right},
		{k.VolumeDown, k.VolumeUp},
		{k.ToggleMute, k.Quit},
	}
}
