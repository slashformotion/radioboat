package tui

import "github.com/charmbracelet/lipgloss"

var (
	statusBarStyle = lipgloss.NewStyle().
			Foreground(lipgloss.AdaptiveColor{Light: "#343433", Dark: "#C1C6B2"}).
			Background(lipgloss.AdaptiveColor{Light: "#D9DCCF", Dark: "#353533"})
	statusStyle = lipgloss.NewStyle().
			Inherit(statusBarStyle).
			Foreground(lipgloss.Color("#FFFDF5")).
			Background(lipgloss.Color("#FF5F87")).
			Padding(0, 1).
			MarginRight(1)
	statusNugget = lipgloss.NewStyle().
			Foreground(lipgloss.Color("#FFFDF5")).
			Padding(0, 1)
	fishCakeStyle = statusNugget.Copy().Background(lipgloss.Color("#6124DF"))
	encodingStyle = statusNugget.Copy().
			Background(lipgloss.Color("#A550DF")).
			Align(lipgloss.Right)
	w          = lipgloss.Width
	width      = 96
	statusText = lipgloss.NewStyle().Inherit(statusBarStyle)
	statusKey  = statusStyle.Render("STATUS")
	encoding   = encodingStyle.Render("UTF-8")
	fishCake   = fishCakeStyle.Render("🍥 Fish Cake")
	statusVal  = statusText.Copy().
			Width(width - w(statusKey) - w(encoding) - w(fishCake)).
			Render("Ravishing")

	bar = lipgloss.JoinHorizontal(lipgloss.Top,
		statusKey,
		statusVal,
		encoding,
		fishCake,
	)
)
