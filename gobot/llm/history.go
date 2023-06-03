package llm

import "strings"

type History struct {
	data []string
	size int
	pos  int
}

func NewHistory(size int) *History {
	return &History{
		data: make([]string, size),
		size: size,
		pos:  0,
	}
}

func 