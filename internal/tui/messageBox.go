package tui

import (
	"fmt"
	"strings"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

var (
	BASE_DELAY    = 5 * time.Second
	message_box_s = lipgloss.NewStyle().
			PaddingLeft(1).
			PaddingRight(1).
			Border(lipgloss.RoundedBorder(), true, true, true, true).
			Align(lipgloss.Center)
	mb_error_s = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#D64B23"))
)

type Message struct {
	msg  string
	err  error
	time time.Time
}

func NewMessage(msg string) *Message {
	return &Message{
		msg:  msg,
		err:  nil,
		time: time.Now().Add(BASE_DELAY),
	}
}
func NewMessageFromErr(err error) *Message {
	return &Message{
		msg:  "",
		err:  err,
		time: time.Now().Add(BASE_DELAY),
	}
}

func (mb *MessageBox) append(msg *Message) {
	mb.content = append(mb.content, msg)
}

type MessageBox struct {
	content []*Message
}

func (mb *MessageBox) View() string {
	var b strings.Builder
	for i, m := range mb.content {
		if m.err != nil {
			b.WriteString(
				mb_error_s.Render(
					fmt.Sprintf("An error happened: %q", m.err.Error())),
			)
		} else {
			b.WriteString(
				lipgloss.NewStyle().Italic(true).Render(
					m.msg,
				))
		}
		if len(mb.content) == 1 {
			continue
		}
		if i != len(mb.content)-1 {
			b.WriteString("\n")
		}
	}
	if len(mb.content) == 0 {
		return ""
	}
	return message_box_s.Render(b.String())

}

func (mb *MessageBox) clean() {
	now := time.Now()
	var newMessageList []*Message
	for _, m := range mb.content {
		if !now.After(m.time) {
			newMessageList = append(newMessageList, m)
		}
	}
	mb.content = newMessageList
}

func (mb *MessageBox) Update(tmsg tea.Msg) tea.Cmd {
	switch tmsg.(type) {
	case TickMessageBox:
		mb.clean()
		return CmdTickerMessageBox
	}
	return nil
}

// wait 1 sec and then send a Tick
func CmdTickerMessageBox() tea.Msg {
	time.Sleep(time.Second)
	return TickMessageBox{}
}

// tea.Msg send by CmdTickerTrackname
type TickMessageBox struct{}
