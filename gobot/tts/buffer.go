
package tts

import (
	"bytes"
	"errors"
)

var (
	ErrBufferFull = errors.New("buffer is already full")
)

// Buffer wraps bytes.Buffer and restricts its maximum size.
type Buffer struct {
	buffer  *bytes.Buffer
	maxSize int
}