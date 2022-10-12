package internal

import (
	"bufio"
	"io"
	"strings"
)

func ReadLine(sr io.Reader) (string, error) {
	line, err := bufio.NewReader(sr).ReadString('\n')
	line = strings.TrimSpace(line)
	return line, err
}
