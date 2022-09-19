package internal_test

import (
	"fmt"
	"log"
	"strings"

	"github.com/aldevv/lang-practice/greet/internal"
)

func ExampleReadLine() {
	sr := strings.NewReader("MyString\r\n")
	line, err := internal.ReadLine(sr)
	if err != nil {
		log.Printf("error reading line!")
		return
	}
	fmt.Printf("%q", line)
	// Output:
	// "MyString"
}
