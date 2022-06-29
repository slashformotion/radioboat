package cmd

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mitchellh/go-homedir"
	"github.com/slashformotion/radioboat/internal/players"
	"github.com/slashformotion/radioboat/internal/tui"
	"github.com/slashformotion/radioboat/internal/urls"
	"github.com/spf13/cobra"
)

var urlFilePath string
var volume int

var rootCmd = &cobra.Command{
	Use:     "radioboat",
	Short:   "Radioboat is a terminal web radio client",
	Long:    `Radioboat is a terminal web radio client, built with simplicity in mind`,
	Version: "v0.0.1",
	Run: func(cmd *cobra.Command, args []string) {
		ui()
	},
}

// rootCmd.PersistentFlags().StringVarP(&Verbose, "verbose", "v", false, "verbose output")

func Execute() {
	hm, err := homedir.Dir()
	if err != nil {
		panic(err)
	}

	rootCmd.PersistentFlags().StringVarP(&urlFilePath, "url-file", "u", hm+"/.config/radioboat/urls.csv", "Use an alternative URL file")
	rootCmd.Flags().IntVar(&volume, "volume", 80, "Set the volume when the application is launched")

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func ui() {
	stations, err := urls.ParseUrlFile(urlFilePath)
	if err != nil {
		panic(err)
	}

	var player players.RadioPlayer = &players.MpvPlayer{}
	p := tea.NewProgram(tui.InitialModel(player, stations, volume), tea.WithAltScreen())
	if err := p.Start(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
