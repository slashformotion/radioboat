package urls

import (
	"os"

	"github.com/gocarina/gocsv"
)

type Station struct { // Our example struct, you can use "-" to ignore a field
	Url  string `csv:"url"`
	Name string `csv:"name"`
	// Age     string `csv:"client_age"`
	NotUsed string `csv:"-"`
}

func ParseUrlFile(filename string) ([]*Station, error) {
	clientsFile, err := os.OpenFile(filename, os.O_RDONLY, os.ModePerm)
	if err != nil {
		return nil, err
	}
	defer clientsFile.Close()

	stations := []*Station{}

	if err := gocsv.UnmarshalFile(clientsFile, &stations); err != nil { // Load clients from file
		panic(err)
	}
	return stations, nil
}
