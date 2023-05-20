package icy_test

import (
	"net/http"
	"testing"

	"github.com/slashformotion/radioboat/icy"
	"github.com/stretchr/testify/assert"
)

// Sample headers
var headers http.Header = map[string][]string{
	"icy-br":              {"192"},
	"icy-country-code":    {"tr"},
	"icy-description":     {"groovy mojito at the beach"},
	"icy-genre":           {"deep house,nu disco,house"},
	"icy-geo-lat-long":    {"41.057849301833365,29.01048472916617"},
	"icy-index-metadata":  {"1"},
	"icy-language-codes":  {"en,tr"},
	"icy-logo":            {"https://www.dinamo.fm/favicons/dinamo-deep-favicon.jpg"},
	"icy-main-stream-url": {"http://channels.dinamo.fm/deep-mp3"},
	"icy-name":            {"dinamo.fm deep"},
	"icy-pub":             {"1"},
	"icy-url":             {"http://www.dinamo.fm"},
}

func TestExtractIcy(t *testing.T) {
	infos := icy.ExtractIcy(headers)

	expected := icy.IcyInfo{
		Bitrate:                192,
		CountryCode:            "tr",
		CountrySubdivisionCode: "",
		Description:            "groovy mojito at the beach",
		Genre:                  "deep house,nu disco,house",
		GeoLat:                 41.057849301833365,
		GeoLong:                29.01048472916617,
		IndexMetadata:          1,
		LanguageCodes:          []string{"en", "tr"},
		Logo:                   "https://www.dinamo.fm/favicons/dinamo-deep-favicon.jpg",
		MainStreamURL:          "http://channels.dinamo.fm/deep-mp3",
		Name:                   "dinamo.fm deep",
		Pub:                    1,
		URL:                    "http://www.dinamo.fm",
	}
	assert.EqualValues(t, expected, infos)
}
