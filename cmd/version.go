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
	"runtime"

	"github.com/spf13/cobra"
)

var verbose bool

// versionCmd represents the version command
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "Print the version to the std output, can be used for debug.",
	Long:  `Print the version to the std output, can be used for debug.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println(BuildVersionString(verbose))
	},
}

func init() {
	versionCmd.Flags().BoolVarP(&verbose, "verbose", "v", false, "level of verbosity")

	rootCmd.AddCommand(versionCmd)

}

var (
	Version string = "unknown"
	// buildDate allows vendor-specified build date when .git/ is unavailable.
	BuildDate string = "unknown"
	// vendorInfo contains vendor notes about the current build.
	VendorInfo string = "unknown"

	// the commit the tool was built on
	Commit string = "unknown"
)

// BuildVersionString creates a version string. This is what you see when
// running "radioboat version".
func BuildVersionString(verbose bool) (versionString string) {

	if verbose {
		versionString = fmt.Sprintf("Radioboat \n\nVersion=%s\nBuildTime=%s\nVendorInfo=%s\nRunOn=%s/%s\nCommit=%s\n", Version, BuildDate, VendorInfo, runtime.GOOS, runtime.GOARCH, Commit)
	} else {
		versionString = fmt.Sprintf("Radioboat Version=%s", Version)
	}

	return versionString
}
