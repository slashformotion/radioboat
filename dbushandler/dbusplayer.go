package dbushandler

import (
	"github.com/godbus/dbus/v5"
	"github.com/quarckster/go-mpris-server/pkg/types"
	"github.com/slashformotion/radioboat/players"
)

func NewDbusPlayer(radioPlayer players.RadioPlayer) DbusPlayer {
	return DbusPlayer{radioPlayer}
}

type DbusPlayer struct {
	radioPlayer players.RadioPlayer
}

var _ types.OrgMprisMediaPlayer2PlayerAdapter = DbusPlayer{}

func (p DbusPlayer) Next() error {
	return nil
}
func (p DbusPlayer) Previous() error {
	return nil
}
func (p DbusPlayer) Pause() error {
	return nil
}
func (p DbusPlayer) PlayPause() error {
	return nil
}
func (p DbusPlayer) Stop() error {
	return nil
}
func (p DbusPlayer) Play() error {
	return nil
}
func (p DbusPlayer) Seek(offset types.Microseconds) error {
	return nil
}
func (p DbusPlayer) SetPosition(trackId string, position types.Microseconds) error {
	return nil
}
func (p DbusPlayer) OpenUri(uri string) error {
	return nil
}
func (p DbusPlayer) PlaybackStatus() (types.PlaybackStatus, error) {
	return types.PlaybackStatusStopped, nil
}
func (p DbusPlayer) Rate() (float64, error) {
	return 0, nil
}
func (p DbusPlayer) SetRate(float64) error {
	return nil
}
func (p DbusPlayer) Metadata() (types.Metadata, error) {
	return types.Metadata{
		TrackId: dbus.ObjectPath(
			p.radioPlayer.NowPlaying(),
		),
		Length:         0,
		ArtUrl:         "",
		Album:          "",
		AlbumArtist:    []string{},
		Artist:         []string{},
		AsText:         "",
		AudioBPM:       0,
		AutoRating:     0.0,
		Comment:        []string{},
		Composer:       []string{},
		ContentCreated: "",
		DiscNumber:     0,
		FirstUsed:      "",
		Genre:          []string{},
		LastUsed:       "",
		Lyricist:       []string{},
		Title:          "",
		TrackNumber:    0,
		Url:            "",
		UseCount:       0,
		UserRating:     0.0,
	}, nil
}
func (p DbusPlayer) Volume() (float64, error) {
	vol := p.radioPlayer.Volume()
	return float64(vol) / 100.0, nil
}
func (p DbusPlayer) SetVolume(newVolume float64) error {
	p.radioPlayer.SetVolume(int(newVolume) * 100)
	return nil
}
func (p DbusPlayer) Position() (int64, error) {
	return 0, nil
}
func (p DbusPlayer) MinimumRate() (float64, error) {
	return 0, nil
}
func (p DbusPlayer) MaximumRate() (float64, error) {
	return 0, nil
}
func (p DbusPlayer) CanGoNext() (bool, error) {
	return false, nil
}
func (p DbusPlayer) CanGoPrevious() (bool, error) {
	return false, nil
}
func (p DbusPlayer) CanPlay() (bool, error) {
	return true, nil
}
func (p DbusPlayer) CanPause() (bool, error) {
	return false, nil
}
func (p DbusPlayer) CanSeek() (bool, error) {
	return false, nil
}
func (p DbusPlayer) CanControl() (bool, error) {
	return true, nil
}
