package utils

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
