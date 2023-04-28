package utils

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
	"io/fs"
	"os"
)

var ErrIsaDirectory = errors.New("the path to the file points to a directory")

// function to check if file exists
func DoesFileExist(fileName string) (bool, error) {
	_, err := os.Stat(fileName)
	if err == nil {
		return true, nil
	} else if errors.Is(err, fs.ErrNotExist) { // check if error is "file not exists"
		return false, nil
	} else {
		return false, err
	}

}
