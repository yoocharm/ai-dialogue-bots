package llm

import "strings"

type History struct {
	data []string
	size int
	pos  int
}

func NewHistory(size int) *History {
	return &History{
		data: mak