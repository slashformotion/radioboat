package icy

import (
	"net/http"
	"strconv"
	"strings"
)

// icy-br: 192
// icy-country-code: tr
// icy-description: groovy mojito at the beach
// icy-genre: deep house,nu disco,house
// icy-geo-lat-long: 41.057849301833365,29.01048472916617
// icy-index-metadata: 1
// icy-language-codes: en,tr
// icy-logo: https://www.dinamo.fm/favicons/dinamo-deep-favicon.jpg
// icy-main-stream-url: http://channels.dinamo.fm/deep-mp3
// icy-name: dinamo.fm deep
// icy-pub: 1
// icy-url: http://www.dinamo.fm

type IcyInfo struct {
	Bitrate                int
	CountryCode            string   // tr
	CountrySubdivisionCode string   // US-NY
	Description            string   // groovy mojito at the beach
	Genre                  string   // deep house,nu disco,house
	GeoLat                 float64  // 41.057849301833365,29.01048472916617
	GeoLong                float64  // 41.057849301833365,29.01048472916617
	IndexMetadata          int      //  1
	LanguageCodes          []string // en,tr
	Logo                   string   // https://www.dinamo.fm/favicons/dinamo-deep-favicon.jpg
	MainStreamURL          string   // http://channels.dinamo.fm/deep-mp3
	Name                   string   //dinamo.fm deep
	Pub                    int      // 1
	URL                    string   // http://www.dinamo.fm
}

func ExtractIcy(headers http.Header) IcyInfo {
	infos := IcyInfo{}
	for header_name, header_values := range headers {
		if len(header_values) < 1 {
			continue
		}
		header_value := header_values[0]

		switch header_name {
		case "icy-br":
			bitrate, err := strconv.Atoi(header_value)
			if err != nil {
				break
			}
			infos.Bitrate = bitrate

		case "icy-country-code":
			infos.CountryCode = header_value

		case "icy-country-subdivision-code":
			if len(header_value) <= 1 {
				infos.CountrySubdivisionCode = header_value
			}
		case "icy-description":
			infos.Description = header_value
		case "icy-genre":
			infos.Genre = header_value
		case "icy-geo-lat-long":
			latitude_str, longitude_str, found := strings.Cut(header_value, ",")
			if !found { // no , found
				break
			}
			latitude, err := strconv.ParseFloat(latitude_str, 64)
			if err != nil {
				break
			}
			longitude, err := strconv.ParseFloat(longitude_str, 64)
			if err != nil {
				break
			}
			infos.GeoLat = latitude
			infos.GeoLong = longitude
		case "icy-index-metadata":
			indexMetadata, err := strconv.Atoi(header_value)
			if err != nil {
				break
			}
			infos.IndexMetadata = indexMetadata
		case "icy-pub":
			pub, err := strconv.Atoi(header_value)
			if err != nil {
				break
			}
			infos.Pub = pub
		case "icy-language-codes":
			infos.LanguageCodes = strings.Split(header_value, ",")
		case "icy-logo":
			infos.Logo = header_value
		case "icy-main-stream-url":
			infos.MainStreamURL = header_value
		case "icy-name":
			infos.Name = header_value
		case "icy-url":
			infos.URL = header_value
		}
	}
	return infos
}
