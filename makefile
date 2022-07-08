# This how we want to name the binary output
BINARY=radioboat

# values to pass for BinVersion, GitCommitLog, GitStatus, BuildTime and BuildGoVersion"
# Version=`git describe --tags`  # git tag 1.0.1  # require tag tagged before

Version=testversion
BuildDate=$(shell date +%FT%T%z)
VendorInfo=$(shell echo $(USER))
Commit=$(shell git log --format="%H" -n 1)
# Setup the -ldflags option for build info here, interpolate the variable values
# notice: replace the path with your versionInfo module path
LDFLAGS=-ldflags "-w -s \
-X github.com/slashformotion/radioboat/internal/buildinfo.Version=${Version} \
-X github.com/slashformotion/radioboat/internal/buildinfo.Commit=${Commit} \
-X github.com/slashformotion/radioboat/internal/buildinfo.BuildDate=${BuildDate} \
-X github.com/slashformotion/radioboat/internal/buildinfo.VendorInfo=${VendorInfo} \
-X main.VendorInfo=${VendorInfo} \
"

all:
	go build -o ${BINARY} ${LDFLAGS}

clean:
	if [ -f ${BINARY} ] ; then rm ${BINARY} ; fi