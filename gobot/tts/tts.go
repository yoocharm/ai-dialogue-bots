
package tts

import (
	"context"
	"io"
	"log"

	"github.com/milosgajdos/go-playht"
)

const (
	MaxTTSBufferSize   = 1000
	defaultQuality     = playht.Low
	defaultOutput      = playht.Mp3
	defaultSpeed       = 1.0
	defaultSampleRate  = 24000
	defaultVoiceEngine = playht.PlayHTv2Turbo
)

type Config struct {
	VoiceID      string
	VoiceEngine  playht.VoiceEngine
	Quality      playht.Quality
	OutputFormat playht.OutputFormat
	Speed        float32
	SampleRate   int32
}

func DefaultConfig() *Config {
	return &Config{
		VoiceID:      "",
		Quality:      defaultQuality,