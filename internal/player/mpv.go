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
	"strings"
	"time"

	mpv "github.com/aynakeya/go-mpv"
	"github.com/slashformotion/radioboat/internal/utils"
)

// MpvPlayer is a wrapper around gompv implementing the RadioPlayer interface
type MpvPlayer struct {
	handle    *mpv.Mpv
	url       string
	done      chan bool
	eventChan chan mpv.Event
}

func NewMpv() *MpvPlayer {
	return &MpvPlayer{
		handle: nil,
		url:    "",
		done:   make(chan bool),
	}
}

const UserRequestID_media_title uint64 = 20000001

func (m *MpvPlayer) Init() (err error) {
	m.handle = mpv.Create()
	err = m.handle.Initialize()
	if err != nil {
		return err
	}
	err = m.handle.ObserveProperty(UserRequestID_media_title, "media-title", mpv.FORMAT_STRING)
	return err
}

// Play stream_url
func (m *MpvPlayer) Play(stream_url string) {
	err := m.handle.Command([]string{"loadfile", stream_url, "replace"})
	if err != nil {
		panic(err)
	}
	m.url = stream_url
}

// Mute set the mute state to true
func (m *MpvPlayer) Mute() {
	err := m.handle.SetProperty("mute", mpv.FORMAT_FLAG, true)
	if err != nil {
		panic(err)
	}
}

// Unmute set the mute state to false
func (m *MpvPlayer) Unmute() {
	err := m.handle.SetProperty("mute", mpv.FORMAT_FLAG, false)
	if err != nil {
		panic(err)
	}
}

// ToggleMute toggle mute state
func (m *MpvPlayer) ToggleMute() {
	if m.IsMute() {
		m.Unmute()
	} else {
		m.Mute()
	}
}

// IsMute return mute state
func (m *MpvPlayer) IsMute() bool {
	mute, err := m.handle.GetProperty("mute", mpv.FORMAT_FLAG)
	if err != nil {
		panic(err)
	}
	return mute.(bool)
}

// IncVolume increase volume by 5%
func (m *MpvPlayer) IncVolume() {
	volume := m.Volume()
	new_volume := utils.ClampInts(volume+5, 0, 110)
	m.SetVolume(new_volume)
}

// Decrease volume by 5%
func (m *MpvPlayer) DecVolume() {
	volume := m.Volume()
	new_volume := utils.ClampInts(volume-5, 0, 110)
	m.SetVolume(new_volume)
}

// SetVolume set the volume to the desired level
func (m *MpvPlayer) SetVolume(volume int) {
	err := m.handle.SetProperty("volume", mpv.FORMAT_INT64, int64(volume))
	if err != nil {
		panic(err)
	}
}

// Return the volume in percentage
func (m *MpvPlayer) Volume() int {
	value, err := m.handle.GetProperty("volume", mpv.FORMAT_INT64)
	if err != nil {
		panic(err)
	}
	return int(value.(int64))
}

// Close the MpvPlayer
func (m *MpvPlayer) Close() {
	m.done <- true // terminating event loop
	close(m.eventChan)
	m.handle.TerminateDestroy()
}

// NowPlaying return a the name of the track playin
func (m *MpvPlayer) NowPlaying() string {
	trackname_interface, _ := m.handle.GetProperty("media-title", mpv.FORMAT_STRING)
	if trackname_interface == nil {
		return ""
	}
	trackname := trackname_interface.(string)
	trackname = strings.TrimSuffix(strings.TrimPrefix(trackname, "\""), "\"")
	// If an URL is in the name of the track, that mean that mpv don't have the name of the track
	// In that case we return an empty string
	if strings.Contains(m.url, trackname) ||
		strings.Contains(strings.ReplaceAll(m.url, "http://", ""), trackname) ||
		strings.Contains(strings.ReplaceAll(m.url, "https://", ""), trackname) {
		return ""
	}
	return trackname
}

func (m *MpvPlayer) Events() chan mpv.Event {
	c := make(chan mpv.Event)
	go func() {
		for {
			select {
			case <-m.done:
				return
			case <-time.After(time.Millisecond):
				e := m.handle.WaitEvent(1)
				if e != nil {
					c <- *e
				}
			}
		}
	}()
	m.eventChan = c
	return c
}
