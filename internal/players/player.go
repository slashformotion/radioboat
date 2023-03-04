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

// ErrPlayerIsNotSupported is returned when player setting is not valid (eg there is no client implementation for that player)
var ErrPlayerIsNotSupported = errors.New("this player is not supported yet, please head to https://github.com/slashformotion/radioboat to see what players are implemented")

// GetPlayer return a RadioPlayer matching the given player name
func GetPlayer(playerName string) (RadioPlayer, error) {
	if playerName == "mpv" {
		return &MpvPlayer{}, nil
	}
	return nil, ErrPlayerIsNotSupported
}

// RadioPayer represent player capable of reading audio streams
type RadioPlayer interface {
	// Init step of the player
	//
	// If an error is returned, radioboat will quit.
	Init() error

	// Play replace the current stream with the new stream URL
	Play(stream_url string)

	// IsMute return mute state
	IsMute() bool
	// Mute set the mute state to true
	Mute()
	// Unmute set the mute state to false
	Unmute()
	// ToggleMute toggle mute state
	ToggleMute()

	// IncVolume volume by 5%
	IncVolume()

	// Decrease decrease the volume by 5%
	DecVolume()

	// SetVolume set the volume to the desired level
	SetVolume(volume int)

	// Volume return the volume in percentage
	Volume() int

	// Close the connection to the player
	Close()

	// NowPlaying return a the name of the track playing
	NowPlaying() string
}
