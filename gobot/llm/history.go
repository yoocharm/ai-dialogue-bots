package llm

import "strings"

type History struct {
	data []string
	size int
	pos  int