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
	"fmt"

	buildinfo "github.com/slashformotion/radioboat/buildinfo"
	"github.com/spf13/cobra"
)

var verbose bool

// versionCmd represents the version command
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the version to the std output, can be used for debug.",
	Long:  `Print the version to the std output, can be used for debug.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println(buildinfo.BuildVersionString(verbose))
	},
}

func init() {
	versionCmd.Flags().BoolVarP(&verbose, "verbose", "v", false, "level of verbosity")

	rootCmd.AddCommand(versionCmd)

}
