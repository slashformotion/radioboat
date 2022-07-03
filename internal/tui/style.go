package tui

import "github.com/charmbracelet/lipgloss"

var (
	header_center_s = lipgloss.NewStyle().
			Foreground(lipgloss.Color("233")).
			Background(lipgloss.Color("147")).
			Align(lipgloss.Center)

	header_status_s = lipgloss.NewStyle().Inherit(header_center_s).
			Foreground(lipgloss.Color("233")).
			Background(lipgloss.Color("#FF5F87")).
			PaddingLeft(1).
			PaddingRight(1)

	header_volume_s = lipgloss.NewStyle().Inherit(header_center_s).
			Foreground(lipgloss.Color("233")).
			Background(lipgloss.Color("#A550DF")).
			PaddingLeft(1).
			PaddingRight(1)

	list_selected_s = lipgloss.NewStyle().Bold(true)

	docStyle = lipgloss.NewStyle().Padding(1, 2, 1, 2)
)
