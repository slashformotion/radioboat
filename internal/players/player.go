package players

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
