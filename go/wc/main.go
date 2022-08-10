package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
)

func count(r io.Reader, lines bool, bytes bool) int {
	// a scanner is used to read text from a reader (such as files)
	scanner := bufio.NewScanner(r)
	// define the scanner split TYPE to words (default is split by lines)
	if !lines && !bytes {
		scanner.Split(bufio.ScanWords)
	}
	if lines {
		scanner.Split(bufio.ScanLines)
	}

	if bytes {
		scanner.Split(bufio.ScanBytes)
	}

	// defining a counter
	wc := 0

	// for every word scanned, increment the counter
	for scanner.Scan() {
		wc++
	}

	// return the total
	return wc
}

func main() {
	lines := flag.Bool("l", false, "Count lines")
	bytes := flag.Bool("b", false, "Count bytes")
	file := os.Args[len(os.Args)-1]
	flag.Parse()

	if file == "wc" {
		fmt.Println(count(os.Stdin, *lines, *bytes))
		return
	}
	openedFile, err := os.OpenFile(file, os.O_RDONLY, os.FileMode(0777))
	if err != nil {
		panic("error reading file")
	}
	fmt.Println(count(openedFile, *lines, *bytes))
}
