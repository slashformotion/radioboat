package players

import (
	"fmt"
)

func Get_player(name string) (RadioPlayer, error) {
	if name == "mpv" {
		return &MpvPlayer{}, nil
	}
	return nil, fmt.Errorf("%q provider does not exists", name)
}

type RadioPlayer interface {
	Init() error
	Play(stream_url string)

	IsMute() bool
	Mute()
	Unmute()
	ToggleMute()
	// Increase volume by 5%
	IncVolume()

	// Decrease volume by 5%
	DecVolume()

	// Set volume
	SetVolume(volume int)

	// Return the volume in percentage
	Volume() int
	Close()
	NowPlaying() string
}
