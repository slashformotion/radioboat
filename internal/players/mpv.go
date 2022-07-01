package players

import (
	"fmt"
	"os/exec"
	"strings"
	"syscall"
	"time"

	"github.com/blang/mpv"
	"github.com/slashformotion/radioboat/internal/utils"
)

type MpvPlayer struct {
	socketname string
	ipcc       *mpv.IPCClient
	client     *mpv.Client
	url        string
}

func (m *MpvPlayer) Init() error {
	m.socketname = utils.RandomString(12)
	cmd := exec.Command("mpv", "--idle",
		fmt.Sprintf("--input-ipc-server=/tmp/%s", m.socketname))
	cmd.SysProcAttr = &syscall.SysProcAttr{
		Pdeathsig: syscall.SIGTERM,
	}
	err := cmd.Start()
	if err != nil {
		return err
	}

	// Waiting to be sure that mpv ipc server is ready
	time.Sleep(400 * time.Millisecond)
	m.ipcc = mpv.NewIPCClient(fmt.Sprintf("/tmp/%s", m.socketname)) // Lowlevel client
	m.client = mpv.NewClient(m.ipcc)
	return nil
}
func (m *MpvPlayer) Play(stream_url string) {
	m.client.Loadfile(stream_url, mpv.LoadFileModeReplace)
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
	m.client.SetProperty("volume", volume)
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
	str = str[1 : len(str)-2]
	if strings.Contains(m.url, str) {
		return ""
	}
	return str
}
