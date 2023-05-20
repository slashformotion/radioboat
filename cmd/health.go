package cmd

import (
	"errors"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/charmbracelet/bubbles/help"
	"github.com/charmbracelet/bubbles/key"
	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/slashformotion/radioboat/internal/tui"
	"github.com/slashformotion/radioboat/internal/urls"
	"github.com/slashformotion/radioboat/internal/utils"
	"github.com/spf13/cobra"
)

// healthCmd represents the health command
var healthCmd = &cobra.Command{
	Use:   "health",
	Short: "Check if the stream urls of your stations are still valids.",
	Long:  `Check if the stream urls of your stations are still valids by making a request to the streaming server. `,
	Run:   health,
}

func init() {
	rootCmd.AddCommand(healthCmd)
}

// main function of the health command
func health(cmd *cobra.Command, args []string) {
	stations, err := urls.ParseUrlFile(urlFilePath)
	if err != nil {
		if os.IsNotExist(err) {
			fmt.Printf("Looks like there is nothing here: %q\n", urlFilePath)
		} else if os.IsPermission(err) {
			fmt.Printf("Looks like you don't have the permission to access the url file: %q\n", urlFilePath)
		} else if errors.Is(err, utils.ErrIsaDirectory) {
			fmt.Printf("Looks like this is a directory: %q\n", urlFilePath)
		} else {
			fmt.Println(err.Error())
		}
		os.Exit(1)
	}

	if len(stations) == 0 {
		log.Fatalf("No stations were found in your url file %q is empty", urlFilePath)
	}
	s := spinner.New()
	s.Spinner = spinner.MiniDot

	p := tea.NewProgram(healthModel{
		startTime: time.Now().UTC(),
		duration:  0,
		stations:  stations,
		spinner:   s,
		help:      help.New(),
	})
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}

}

// A result after a check
type Result struct {
	station urls.Station
	valid   bool
	err     string
}

// bubbletea model
type healthModel struct {
	startTime time.Time
	duration  time.Duration
	stations  []*urls.Station
	errors    []Result
	valids    []Result
	spinner   spinner.Model
	help      help.Model
}

// Finished teaMsg Type
type finishedMsg struct{}

func (m healthModel) Update(tmsg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := tmsg.(type) {
	case finishedMsg:
		return m, tea.Quit
	case Result:
		if msg.valid {
			m.valids = append(m.valids, msg)
		} else {
			m.errors = append(m.errors, msg)
		}
		if len(m.stations) == len(m.valids)+len(m.errors) {
			m.duration = time.Since(m.startTime)
			return m, func() tea.Msg {
				return finishedMsg{}
			}
		}
		return m, nil
	case tea.KeyMsg:
		switch {
		case key.Matches(msg, tui.DefaultKeyMap.Quit):
			return m, tea.Quit
		}
	case spinner.TickMsg:
		var cmd tea.Cmd

		m.spinner, cmd = m.spinner.Update(msg)
		return m, cmd
	case tea.WindowSizeMsg:
		m.help.Width = msg.Width
	}
	return m, nil
}

func (m healthModel) count() int { return len(m.valids) + len(m.errors) }

func (m healthModel) View() string {
	out := ""
	if len(m.stations) != m.count() {
		out += m.spinner.View() + fmt.Sprintf(" %d results pending\n\n", len(m.stations)-m.count())
	} else {
		out += fmt.Sprintf("Finished in %s \n\n", m.duration.String())
	}
	out += "Errors:\n"
	out += createErrorGrid(m)
	out += "\n"
	out += fmt.Sprintf("%d healthy stations, %d errored\n", len(m.valids), len(m.errors))
	out += m.help.View(healthKeymap)

	return out
}

func createErrorGrid(m healthModel) string {
	names := ""
	errs := ""
	for _, res := range m.errors {
		names += "- " + res.station.Name + "\n"
		errs += fmt.Sprintf(" : %s", res.err)

	}

	return lipgloss.JoinHorizontal(lipgloss.Top, names, errs)
}

func (m healthModel) Init() tea.Cmd {
	cmds := make([]tea.Cmd, 0)
	for _, s := range m.stations {
		cmds = append(cmds, makeCmd(*s))
	}
	cmds = append(cmds, m.spinner.Tick)

	return tea.Batch(cmds...)
}

func makeCmd(station urls.Station) tea.Cmd {
	return func() tea.Msg {
		msg := Result{
			station: station,
			valid:   false,
		}
		resp, err := http.Get(station.Url)
		if err != nil {
			msg.err = err.Error()
			return msg
		}
		defer resp.Body.Close()
		if resp.StatusCode >= 200 && resp.StatusCode < 300 {
			msg.valid = true
		} else {
			msg.err = fmt.Sprintf("got %d", resp.StatusCode)
		}
		return msg
	}
}

type KeyMap struct {
	Quit key.Binding
}

var healthKeymap = KeyMap{

	Quit: key.NewBinding(
		key.WithKeys("ctrl+c", "esc", "q"),
		key.WithHelp("Ctrl+C/Escape/q", "Quit"),
	),
}

// ShortHelp returns keybindings to be shown in the mini help view. It's part
// of the key.Map interface.
func (k KeyMap) ShortHelp() []key.Binding {
	return []key.Binding{k.Quit}
}

// FullHelp returns keybindings for the expanded help view. It's part of the
// key.Map interface.
func (k KeyMap) FullHelp() [][]key.Binding {
	return [][]key.Binding{
		{k.Quit},
	}
}
