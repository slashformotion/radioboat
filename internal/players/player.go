package players

import (
	"errors"
)

var ErrPlayerIsNotSupported = errors.New("this player is not supported yet, please head to https://github.com/slashformotion/radioboat to see what players are implemented")

func Get_player(name string) (RadioPlayer, error) {
	if name == "mpv" {
		return &MpvPlayer{}, nil
	}
	return nil, ErrPlayerIsNotSupported
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
