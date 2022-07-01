package urls

import (
	"errors"
	"os"
	"strings"

	"github.com/gocarina/gocsv"
)

type Station struct { // Our example struct, you can use "-" to ignore a field
	Url  string `csv:"url"`
	Name string `csv:"name"`
	// Age     string `csv:"client_age"`
	NotUsed string `csv:"-"`
}

var ErrIsaDirectory = errors.New("the path to the file points to a directory")

func ParseUrlFile(filename string) ([]*Station, error) {
	stations := []*Station{}
	// Check if filename is a directory
	stat, err := os.Stat(filename)
	if err != nil {
		return stations, err
	}
	if stat.IsDir() {
		return stations, ErrIsaDirectory
	}

	clientsFile, err := os.OpenFile(filename, os.O_RDONLY, os.ModePerm)
	if err != nil {
		return nil, err
	}
	defer clientsFile.Close()

	if err := gocsv.UnmarshalFile(clientsFile, &stations); err != nil { // Load clients from file
		panic(err)
	}
	for _, s := range stations {
		s.Url = strings.TrimSpace(s.Url)
		s.Name = strings.TrimSpace(s.Name)
	}
	return stations, nil
}
