package tui

import "github.com/charmbracelet/lipgloss"

var (
	color_header = lipgloss.AdaptiveColor{Light: "236", Dark: "248"}

	header_s = lipgloss.NewStyle().
		// PaddingTop(1).
		// PaddingRight(1).
		// PaddingBottom(1).
		// PaddingLeft(1).
		Foreground(lipgloss.Color("233")).
		Background(lipgloss.Color("147")).
		Blink(true).
		// BorderTop(true).
		// BorderStyle(lipgloss.NormalBorder()).
		Align(lipgloss.Center)

	list_selected_s = lipgloss.NewStyle().Bold(true)

	docStyle = lipgloss.NewStyle().Padding(1, 2, 1, 2)
)
