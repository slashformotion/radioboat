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
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

// lsCmd represents the ls command
var lsCmd = &cobra.Command{
	Use:   "edit",
	Short: "Edit the urls.csv file in $RADIOBOAT_EDITOR or $EDITOR or vim",
	Long: `Edit the urls.csv file in $RADIOBOAT_EDITOR or $EDITOR or vim. 

You need to have correct environment variables set up. 
To do that please head to the wiki: https://github.com/slashformotion/radioboat/wiki/Configuration`,
	Run: func(cmd *cobra.Command, args []string) {
		// editor = osutil.GetOptEnv("EDITOR"))
		// cm := exec.Command("nvim", urlFilePath, "</dev/tty")
		prefEditor := os.Getenv("RADIOBOAT_EDITOR")
		editor := os.Getenv("EDITOR")
		if prefEditor != "" {
			editor = prefEditor
		} else if editor == "" {
			editor = "vim"
		}
		cm := exec.Command(editor, urlFilePath)
		cm.Stdin = os.Stdin
		cm.Stdout = os.Stdout
		cm.Stderr = os.Stderr
		cm.Start()
		cm.Wait()
	},
}

func init() {
	rootCmd.AddCommand(lsCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// lsCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// lsCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
