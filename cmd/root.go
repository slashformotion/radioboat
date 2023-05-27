package cmd

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import (
	"errors"
	"fmt"
	"log"
	"os"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/mitchellh/go-homedir"
	players "github.com/slashformotion/radioboat/internal/player"
	"github.com/slashformotion/radioboat/internal/tui"
	"github.com/slashformotion/radioboat/internal/urls"
	"github.com/slashformotion/radioboat/internal/utils"
	"github.com/spf13/cobra"
)

var urlFilePath string
var volume int
var trackFilePath string

var rootCmd = &cobra.Command{
	Use:   "radioboat",
	Short: "Radioboat is a terminal web radio client",
	Long:  `Radioboat is a terminal web radio client, built with simplicity in mind`,
	Run: func(cmd *cobra.Command, args []string) {
		ui()
	},
}

func Execute() {
	hm, err := homedir.Dir()
	if err != nil {
		panic(err)
	}

	rootCmd.PersistentFlags().StringVarP(&urlFilePath, "url-file", "u", hm+"/.config/radioboat/urls.csv", "Use an alternative URL file")
	rootCmd.Flags().IntVar(&volume, "volume", 80, "Set the volume when the application is launched")
	rootCmd.Flags().StringVarP(&trackFilePath, "track-file", "t", hm+"/.tracks", "Use an alternative track text file")

	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func ui() {
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

	stat, err := os.Stat(trackFilePath)
	if err != nil {
		if os.IsNotExist(err) {
			fmt.Printf("The track file %q does not exist.\n", trackFilePath)
			prompt, err := utils.GetInteractiveBooleanPrompt("Do you want radioboat to create a trackfile at this location ?")
			if err != nil {
				panic(err)
			}
			var res string
			for {
				res, err = prompt.Run()
				if err != nil {
					fmt.Println("Please answer again.")
				}
				if res == "n" {
					fmt.Println("Exiting now without creating the trackfile.")
					return
				} else {
					break
				}
			}
			file, err := os.Create(trackFilePath)
			if err != nil {
				panic(err)
			}
			err = file.Close()
			if err != nil {
				panic(err)
			}
		} else if os.IsPermission(err) {
			fmt.Printf("Looks like you don't have the permission to access the track-file: %q\n", trackFilePath)
			os.Exit(1)
		}
	} else if stat.IsDir() {
		fmt.Printf("Looks like this is a directory: %q\n", trackFilePath)
		os.Exit(1)
	}
	player := players.NewMpv()
	err = player.Init()
	if err != nil {
		log.Fatal(err)
	}
	chanEvent := player.Events()

	defer player.Close()
	p := tea.NewProgram(tui.InitialModel(player, stations, volume, trackFilePath, chanEvent), tea.WithAltScreen())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
	fmt.Println("Exiting...")
}
