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

func (h *History) Add(element string) {
	h.data[h.pos] = element
	h.pos = (h.pos + 1) % h.size
}

func (h *History) String() string {
	sb := strings.Builder{}
	