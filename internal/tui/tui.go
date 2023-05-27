package tui

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// #include <mpv/client.h>
// #include <stdlib.h>
import "C"
import (
	"fmt"
	"os"

	mpv "github.com/aynakeya/go-mpv"
	tm "github.com/buger/goterm"
	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/slashformotion/radioboat/internal/players"
	"github.com/slashformotion/radioboat/internal/urls"
)

var (
	// width of the terminal
	width int
	// height of the terminal
	height int

	// height of the main widget
	centerHeight int
)

type model struct {
	savedTracks []string
	stations    []*urls.Station
	cursor      int
	player      *players.MpvPlayer
	help        help.Model

	currentStation string
	muted          bool
	volume         int
	currentTrack   string

	trackFilePath string
	mb            *MessageBox

	chanEvent chan mpv.Event
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
	return tea.Batch(CmdTickerMessageBox, waitForMpvEvent(m.chanEvent))
}

func log(i ...any) {
	f, _ := tea.LogToFile("log.log", "")
	fmt.Fprintln(f, i...)
	f.Close()
}

func (m model) Update(tmsg tea.Msg) (tea.Model, tea.Cmd) {

	switch msg := tmsg.(type) {
	case TickMessageBox:
		return m, m.mb.Update(tmsg)
	case SaveTrackMsg:
		if msg.err != nil {
			m.mb.append(
				NewMessageFromErr(msg.err),
			)
			return m, nil
		} else {
			m.mb.append(
				NewMessage(fmt.Sprintf("Just saved track name %q to %q", msg.trackName, m.trackFilePath)),
			)
		}
	case mpv.Event:
		switch msg.EventId {
		case mpv.EVENT_PROPERTY_CHANGE:
			if msg.ReplyUserData == players.UserRequestID_media_title {
				trackName, ok := msg.Property().Data.(string)
				if ok {
					m.currentTrack = trackName
				} else {
					m.currentTrack = ""
				}
			}
		}
		return m, waitForMpvEvent(m.chanEvent)

	case tea.WindowSizeMsg:
		width = msg.Width
		height = msg.Height
		m.help.Width = msg.Width

	case tea.KeyMsg:
		switch {
		case key.Matches(msg, DefaultKeyMap.Help):
			m.help.ShowAll = !m.help.ShowAll
		case key.Matches(msg, DefaultKeyMap.Quit):
			return m, tea.Quit
		case key.Matches(msg, DefaultKeyMap.Up):
			m.cursor--
			if m.cursor < 0 {
				m.cursor = len(m.stations) - 1
			}
		case key.Matches(msg, DefaultKeyMap.Down):
			m.cursor++
			if m.cursor >= len(m.stations) {
				m.cursor = 0
			}
		case key.Matches(msg, DefaultKeyMap.Left):
			m.cursor -= centerHeight
			if m.cursor < 0 {
				m.cursor = 0
			}
		case key.Matches(msg, DefaultKeyMap.Right):
			m.cursor += centerHeight
			if m.cursor >= len(m.stations) {
				m.cursor = len(m.stations) - 1
			}
		case key.Matches(msg, DefaultKeyMap.ToggleMute):
			m.player.ToggleMute()
			m.muted = m.player.IsMute()
		case key.Matches(msg, DefaultKeyMap.Play):
			m.player.Play(m.stations[m.cursor].Url)
			m.currentStation = m.stations[m.cursor].Name

		case key.Matches(msg, DefaultKeyMap.VolumeUp):
			m.player.IncVolume()
			m.volume = m.player.Volume()
		case key.Matches(msg, DefaultKeyMap.VolumeDown):
			m.player.DecVolume()
			m.volume = m.player.Volume()
		case key.Matches(msg, DefaultKeyMap.SaveTrack):
			trackName := m.player.NowPlaying()
			for _, s := range m.savedTracks {
				if trackName == s {
					return m, nil
				}
			}
			m.savedTracks = append(m.savedTracks, trackName)
			return m, CmdSaveTrack(m.trackFilePath, trackName)
		}
	}

	return m, nil
}

func (m model) View() string {
	header := HeaderToString(m.currentStation, m.currentTrack, m.volume, m.muted)
	messagebox := m.mb.View()
	helpView := m.help.View(DefaultKeyMap)
	centerHeight = height - lipgloss.Height(header) - lipgloss.Height(messagebox) - lipgloss.Height(helpView) - 3
	// centerHeight = 22
	s := header
	// Iterate over our choices
	nbColumns := len(m.stations)/centerHeight + 1
	columns := make([]string, nbColumns)
	for i, station := range m.stations {

		cursor := "   " // no cursor
		name := station.Name
		if m.cursor == i {
			cursor = " ➤ " // cursor!
			name = list_selected_s.Render(station.Name)
		}
		if m.stations[i].Name == m.currentStation {
			name = list_selected_s.Copy().Italic(true).Bold(false).Render(station.Name)
		}
		var columnNumber uint = uint(i / centerHeight)
		line := cursor + name
		columns[columnNumber] += line + "\n"

	}
	s += lipgloss.JoinHorizontal(lipgloss.Left, columns...)
	s += "\n" + messagebox + "\n" + helpView
	return docStyle.Render(s)
}

func InitialModel(p *players.MpvPlayer, stations []*urls.Station, volume int, trackFilePath string, chanEvent chan mpv.Event) model {
	m := model{
		player:         p,
		stations:       stations,
		currentStation: "Not Playing",
		volume:         volume,
		help:           help.New(),
		trackFilePath:  trackFilePath,
		mb:             new(MessageBox),
		chanEvent:      chanEvent,
	}
	m.player.SetVolume(volume)
	m.volume = m.player.Volume()
	m.muted = m.player.IsMute()
	height = tm.Height()
	width = tm.Width()
	return m
}

// A command that waits for the activity on a channel.
func waitForMpvEvent(chanEvent chan mpv.Event) tea.Cmd {
	return func() tea.Msg {
		return <-chanEvent
	}
}

func CmdSaveTrack(trackFilePath, track string) tea.Cmd {
	return func() tea.Msg {
		var msg SaveTrackMsg = SaveTrackMsg{err: nil}
		if track == "" {
			return msg
		}
		trackFile, err := os.OpenFile(trackFilePath, os.O_APPEND|os.O_WRONLY, os.ModePerm)
		if err != nil {
			msg.err = err
			return msg
		}
		_, err = fmt.Fprintf(trackFile, "%s\n", track)
		if err != nil {
			msg.err = err
			return msg
		}
		msg.err = trackFile.Close()
		msg.trackName = track
		return msg
	}
}

// tea.Msg send by CmdSaveTrack
type SaveTrackMsg struct {
	err       error
	trackName string
}
