package cmd

import (
	"os"
	"os/exec"

	"github.com/spf13/cobra"
)

// lsCmd represents the ls command
var lsCmd = &cobra.Command{
	Use:   "edit",
	Short: "Edit the urls.csv file in $EDITOR",
	Long: `Edit the urls.csv file in $EDITOR. 
You need to have the EDITOR env variable set up.`,
	Run: func(cmd *cobra.Command, args []string) {
		// editor = osutil.GetOptEnv("EDITOR"))
		// cm := exec.Command("nvim", urlFilePath, "</dev/tty")
		cm := exec.Command("nvim", urlFilePath)
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
