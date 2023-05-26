package dbus

import (
	"log"
	"strings"

	"github.com/godbus/dbus/v5"
	"github.com/godbus/dbus/v5/introspect"
	"github.com/godbus/dbus/v5/prop"
	"github.com/slashformotion/radioboat/internal/players"
)

type MetadataMap map[string]interface{}

type DbusInstance struct {
	props *prop.Properties
	conn  *dbus.Conn
}

func CreateDbusInstance(mpv players.RadioPlayer) *DbusInstance {
	conn, err := dbus.ConnectSessionBus()
	if err != nil {
		log.Fatalln(err)
	}

	ins := &DbusInstance{
		conn: conn,
	}
	mp2 := &MediaPlayer2{ins: ins, mpv: mpv}

	err = conn.Export(mp2, "/org/mpris/MediaPlayer2", "org.mpris.MediaPlayer2.Player")
	if err != nil {
		log.Fatalln(err)
	}

	err = conn.Export(introspect.NewIntrospectable(IntrospectNode()), "/org/mpris/MediaPlayer2", "org.freedesktop.DBus.Introspectable")
	if err != nil {
		log.Fatalln(err)
	}

	ins.props, err = prop.Export(conn, "/org/mpris/MediaPlayer2", map[string]map[string]*prop.Prop{
		"org.mpris.MediaPlayer2":        mp2.properties(),
		"org.mpris.MediaPlayer2.Player": mp2.playerProps(),
	})
	if err != nil {
		log.Fatalln(err)
	}

	reply, err := conn.RequestName("org.mpris.MediaPlayer2.radioplayer", dbus.NameFlagReplaceExisting)
	if err != nil {
		log.Fatalln(err)
	}
	if reply != dbus.RequestNameReplyPrimaryOwner {
		log.Fatalln("Name already taken")
	}

	return ins
}

func GetMetadataMap(metadata Metadata) MetadataMap {
	id := strings.ReplaceAll(metadata.Title, "-", "")

	var trackId string
	if id == "" {
		trackId = "/org/mpris/MediaPlayer2/TrackList/NoTrack"
	} else {
		trackId = "/org/mpris/MediaPlayer2/" + id
	}

	m := &MetadataMap{
		"mpris:trackid": dbus.ObjectPath(trackId),
		// "mpris:length":  fm.Duration(),
		// "mpris:artUrl":  fm.Now.Cover.Src,

		// "xesam:album":          fm.Now.Song.Release.Title,
		// "xesam:albumArtist":    []string{fm.Now.SecondLine},
		// "xesam:artist":         []string{fm.Now.SecondLine},
		// "xesam:contentCreated": fm.ContentCreated(),
		// "xesam:title":          fm.Now.FirstLine,
	}

	return *m
}

func UpdateMetadata(ins *DbusInstance, md Metadata) {
	metadata := GetMetadataMap(md)

	dbusErr := ins.props.Set(
		"org.mpris.MediaPlayer2.Player",
		"Metadata",
		dbus.MakeVariant(metadata),
	)
	if dbusErr != nil {
		log.Println(dbusErr, metadata)
	}
}

func (ins *DbusInstance) CloseConnection() {
	ins.conn.Close()
}
