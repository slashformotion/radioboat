package urls_test

import (
	"strings"
	"testing"

	"github.com/slashformotion/radioboat/urls"
	"github.com/stretchr/testify/assert"
)

const sampleURLS = `url,"name"
https://walmradio.com:8443/jazz,jazz
http://channels.dinamo.fm/deep-mp3,deep
http://prem2.classicalradio.com/violinworks,violin`

func TestParseURLS(t *testing.T) {
	reader := strings.NewReader(sampleURLS)
	stationUrls, err := urls.ParseURLS(reader)
	assert.NoError(t, err)
	assert.Len(t, stationUrls, 3)
	assert.ElementsMatch(t, stationUrls, []*urls.Station{
		{
			"https://walmradio.com:8443/jazz",
			"jazz",
		},
		{
			"http://channels.dinamo.fm/deep-mp3",
			"deep",
		},
		{
			"http://prem2.classicalradio.com/violinworks",
			"violin",
		},
	})

}
