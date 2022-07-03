package tui

import (
	"fmt"

	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/slashformotion/radioboat/internal/players"
	"github.com/slashformotion/radioboat/internal/urls"
)

var width int
var height int

type model struct {
	stations []*urls.Station
	cursor   int
	player   players.RadioPlayer
	help     help.Model
	dj       Dj
}

type Dj struct {
	currentStation string
	muted          bool
	volume         int
}

func HeaderToString(currentStation string, trackName string, volume int, muted bool) string {
	var mutedStr string
	if muted {
		mutedStr = fmt.Sprintf("Muted(%d)", volume)
	} else {
		mutedStr = fmt.Sprintf("Volume %d", volume)
	}
	statusStr := header_status_s.Render(currentStation)
	volumeStr := header_volume_s.Render(mutedStr)
	centerStr := header_center_s.Copy().
		Width(width - lipgloss.Width(statusStr) - lipgloss.Width(volumeStr) - 3). // -3 because of the doc margin
		Render(trackName)
	s := lipgloss.JoinHorizontal(lipgloss.Top, statusStr, centerStr, volumeStr)
	s += "\n\n"
	return s
}

func (m model) Init() tea.Cmd {
	// Just return `nil`, which means "no I/O right now, please."
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.WindowSizeMsg:
		width = msg.Width
		height = msg.Height
		m.help.Width = msg.Width

	case tea.KeyMsg:
		switch {
		case key.Matches(msg, DefaultKeyMap.Quit):
			return m, tea.Quit
		case key.Matches(msg, DefaultKeyMap.Up):
			m.cursor--
			if m.cursor < 0 {
				m.cursor = 0
			}
		case key.Matches(msg, DefaultKeyMap.Down):
			m.cursor++
			if m.cursor >= len(m.stations) {
				m.cursor = len(m.stations) - 1
			}
		case key.Matches(msg, DefaultKeyMap.ToggleMute):
			m.player.ToggleMute()
			m.dj.muted = m.player.IsMute()
		case key.Matches(msg, DefaultKeyMap.Play):
			m.player.Play(m.stations[m.cursor].Url)
			m.dj.currentStation = m.stations[m.cursor].Name

		case key.Matches(msg, DefaultKeyMap.VolumeUp):
			m.player.IncVolume()
			m.dj.volume = m.player.Volume()
		case key.Matches(msg, DefaultKeyMap.VolumeDown):
			m.player.DecVolume()
			m.dj.volume = m.player.Volume()
		}
	}

	return m, nil
}

func (m model) View() string {
	s := HeaderToString(m.dj.currentStation, m.player.NowPlaying(), m.dj.volume, m.dj.muted)
	// Iterate over our choices
	for i, station := range m.stations {

		cursor := " " // no cursor
		name := station.Name
		if m.cursor == i {
			cursor = ">" // cursor!
			name = list_selected_s.Render(station.Name)
		}
		if m.dj.currentStation == station.Name {
			name = list_selected_s.Copy().Italic(true).Bold(false).Render(station.Name)
		}

		s += fmt.Sprintf("%s %s\n", cursor, name)
	}
	helpView := m.help.View(DefaultKeyMap)
	s += "\n\n" + helpView

	return docStyle.Render(s)
}
func InitialModel(p players.RadioPlayer, stations []*urls.Station, volume int) model {
	m := model{
		player:   p,
		stations: stations,
		dj: Dj{
			currentStation: "Not Playing",
			volume:         volume,
		},
		help: help.New(),
	}
	m.player.Init()
	m.player.SetVolume(volume)
	m.dj.volume = m.player.Volume()
	m.dj.muted = m.player.IsMute()

	// m.help.ShowAll = true
	return m
}
