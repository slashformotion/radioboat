package players

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
}
