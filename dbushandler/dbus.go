package dbushandler

import (
	"github.com/quarckster/go-mpris-server/pkg/server"
	"github.com/quarckster/go-mpris-server/pkg/types"
	"github.com/slashformotion/radioboat/players"
)

type Root struct{}

// Implement other methods of `pkg.types.OrgMprisMediaPlayer2Adapter`
func (r Root) Raise() error {
	return nil
}

func (r Root) Quit() error {
	return nil
}
func (r Root) CanQuit() (bool, error) {
	return false, nil
}
func (r Root) CanRaise() (bool, error) {
	return false, nil
}
func (r Root) HasTrackList() (bool, error) {
	return false, nil
}
func (r Root) Identity() (string, error) {
	return "Radioboat", nil
}
func (r Root) SupportedUriSchemes() ([]string, error) {
	return []string{}, nil
}
func (r Root) SupportedMimeTypes() ([]string, error) {
	return []string{}, nil
}

var _ types.OrgMprisMediaPlayer2Adapter = Root{}

func MakeDbusServer(radioPlayer players.RadioPlayer) *server.Server {
	root := Root{}
	player := NewDbusPlayer(radioPlayer)
	return server.NewServer("radioboat", root, player)
}
