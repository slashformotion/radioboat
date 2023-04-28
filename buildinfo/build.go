package buildinfo

import (
	"fmt"
	"runtime"
)

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
