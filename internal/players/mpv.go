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
	"fmt"
	"net"
	"os/exec"
	"strings"
	"syscall"
	"time"

	"github.com/jpillora/backoff"
	mpv "github.com/slashformotion/gompv"
	"github.com/slashformotion/radioboat/internal/utils"
)

type MpvPlayer struct {
	ipcc   *mpv.IPCClient
	client *mpv.Client
	url    string
}

func (m *MpvPlayer) Init() error {
	socketpath := "/tmp/radioboat-" + utils.RandomString(25)
	cmd := exec.Command("mpv", "--idle", "--no-video",
		fmt.Sprintf("--input-ipc-server=%s", socketpath))
	// Bind the child process to radioboat process. When radioboat exit, mpv will receive a SIGTERM
	cmd.SysProcAttr = &syscall.SysProcAttr{
		Pdeathsig: syscall.SIGTERM,
	}

	retry := backoff.Backoff{
		Factor: 3,
		Jitter: false,
		Min:    10 * time.Millisecond,
		Max:    2 * time.Second,
	}
	// starting mpv
	err := cmd.Start()
	if err != nil {
		return err
	}
	// Exponential backoff
	var numberOfTimeConnectionTried uint
	for {
		numberOfTimeConnectionTried += 1
		conn, err := net.Dial("unix", socketpath)
		if err != nil {
			currentSpleepTime := retry.Duration()
			if currentSpleepTime == retry.Max {
				return fmt.Errorf(
					"failed to connect to mpv (socketpath=%s, tried to connect %d time)",
					socketpath,
					numberOfTimeConnectionTried)
			}
			time.Sleep(currentSpleepTime)
			continue
		}
		conn.Close()
		break
	}

	// Waiting to be sure that mpv ipc server is ready
	m.ipcc = mpv.NewIPCClient(socketpath) // Lowlevel client
	m.client = mpv.NewClient(m.ipcc)
	return nil
}
func (m *MpvPlayer) Play(stream_url string) {
	err := m.client.Loadfile(stream_url, mpv.LoadFileModeReplace)
	if err != nil {
		panic(err)
	}
	m.url = stream_url
}
func (m *MpvPlayer) Mute() {
	err := m.client.SetMute(true)
	if err != nil {
		panic(err)
	}
}
func (m *MpvPlayer) Unmute() {
	err := m.client.SetMute(false)
	if err != nil {
		panic(err)
	}

}
func (m *MpvPlayer) ToggleMute() {
	mute, err := m.client.Mute()
	if err != nil {
		panic(err)
	}
	if mute {
		m.Unmute()
	} else {
		m.Mute()
	}
}

func (m *MpvPlayer) IsMute() bool {
	mute, err := m.client.Mute()
	if err != nil {
		panic(err)
	}
	return mute
}

// Increase volume by 5%
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

// Set volume
func (m *MpvPlayer) SetVolume(volume int) {
	err := m.client.SetProperty("volume", volume)
	if err != nil {
		panic(err)
	}
}

// Return the volume in percentage
func (m *MpvPlayer) Volume() int {
	value, err := m.client.Volume()
	if err != nil {
		panic(err)
	}
	return int(value)

}
func (m *MpvPlayer) Close() {

}

func (m *MpvPlayer) NowPlaying() string {
	str, _ := m.client.GetProperty("media-title")
	if str == "<nil>" {
		return ""
	}
	str = str[1 : len(str)-1]
	if strings.Contains(m.url, str) ||
		strings.Contains(strings.ReplaceAll(m.url, "http://", ""), str) ||
		strings.Contains(strings.ReplaceAll(m.url, "https://", ""), str) {
		return ""
	}
	return str
}
