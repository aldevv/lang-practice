package main

import (
	"fmt"
	"log"
	"os"
	"strings"

	"github.com/aldevv/lang-practice/greet"
	"github.com/aldevv/lang-practice/greet/internal"
)

func main() {
	var name string
	var err error
	// get as an argument
	if len(os.Args) > 1 {
		name = strings.Join(os.Args[1:], " ")
	}

	// get as an env variable
	if name == "" {
		name, _ = os.LookupEnv("NOMBRE")
	}

	// get from stdin
	if name == "" {
		fmt.Println("Entra tu nombre:")
		name, err = internal.ReadLine(os.Stdin)
		if err != nil {
			log.Println("Problema leyendo desde stdin")
			return
		}
	}
	greet.Buenas(name)
}
