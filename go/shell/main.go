package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"os/user"
	"strings"
)

const (
	cd   string = "cd"
	exit string = "exit"
)

func execInput(input string) error {
	// Remove the newline character.
	input = strings.TrimSuffix(input, "\n")
	args := strings.Split(input, " ")

	// execute the command stored in input,
	// with it's arguments using a spread operator

	if isBuiltin(args[0]) {
		return execBuiltin(args)
	}

	cmd := exec.Command(args[0], args[1:]...)

	// set the descriptors that the cmd is going to use
	cmd.Stderr = os.Stderr
	cmd.Stdout = os.Stdout

	return cmd.Run()
}

func isBuiltin(command string) bool {
	switch command {
	case cd:
		return true
	case exit:
		return true
	}
	return false
}

func execBuiltin(args []string) error {
	switch args[0] {
	case cd:
		if len(args) < 2 {
			return errors.New("path required")
		}
		return os.Chdir(args[1])
	case exit:
		os.Exit(0)
	}
	return nil
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	// read until enter is pressed
	for {
		user, err := user.Current()
		hostname, err := os.Hostname()
		fmt.Print("[" + user.Username + "@" + hostname + "]\n$ ")
		input, err := reader.ReadString('\n')
		if err != nil {
			// Fprintln to print to Stderr
			fmt.Fprintln(os.Stderr, err)
		}

		// execute the input
		if err = execInput(input); err != nil {
			fmt.Fprintln(os.Stderr, err)
		}
	}

}
