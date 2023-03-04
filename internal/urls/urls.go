package urls

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
	"io"
	"os"
	"strings"

	"github.com/gocarina/gocsv"
	"github.com/slashformotion/radioboat/internal/utils"
)

// Station represent a single webradio station
type Station struct {
	Url     string `csv:"url"`
	Name    string `csv:"name"`
	NotUsed string `csv:"-"`
}

// Station
func ParseUrlFile(filename string) ([]*Station, error) {
	stat, err := os.Stat(filename)
	if err != nil {
		return nil, err
	}

	// Check if filename is a directory
	if stat.IsDir() {
		return nil, utils.ErrIsaDirectory
	}

	clientsFile, err := os.OpenFile(filename, os.O_RDONLY, os.ModePerm)
	if err != nil {
		return nil, err
	}

	// Load clients from file
	stations, err := ParseURLS(clientsFile)
	if err != nil {
		return nil, err
	}
	return stations, clientsFile.Close()
}

// ParseURLS parse the url from a [io.ReadCloser]
func ParseURLS(in io.Reader) ([]*Station, error) {
	stations := []*Station{}
	if err := gocsv.Unmarshal(in, &stations); err != nil {
		return stations, fmt.Errorf("failed to decode url file, err=%s", err.Error())
	}
	for _, s := range stations {
		s.Url = strings.TrimSpace(s.Url)
		s.Name = strings.TrimSpace(s.Name)
	}
	return stations, nil
}
