package main

import (
	"fmt"
	"log"

	"github.com/neovim/go-client/nvim"
)

func main() {
	// printBuffers()
	printHelloWorld()
}

func printBuffers() {
	// Get address from environment variable set by Nvim.

	addr := "127.0.0.1:6666"

	// Dial with default options.
	v, err := nvim.Dial(addr)
	if err != nil {
		log.Fatal(err)
	}

	// Cleanup on return.
	defer v.Close()

	bufs, err := v.Buffers()
	if err != nil {
		log.Fatal(err)
	}

	// Get the names using a single atomic call to Nvim.
	names := make([]string, len(bufs))
	b := v.NewBatch()
	for i, buf := range bufs {
		b.BufferName(buf, &names[i])
	}
	if err := b.Execute(); err != nil {
		log.Fatal(err)
	}

	// Print the names.
	for _, name := range names {
		fmt.Println(name)
	}
}

func printHelloWorld() {

	// Get address from environment variable set by Nvim.
	addr := "127.0.0.1:6666"

	// Dial with default options.
	v, err := nvim.Dial(addr)
	if err != nil {
		log.Fatal(err)
	}

	// Cleanup on return.
	defer v.Close()

	var res any
	err = v.ExecLua("print('hello world')", &res)
	if err != nil {
		log.Fatal(err)
	}
	// __AUTO_GENERATED_PRINT_VAR_START__
	fmt.Println(fmt.Sprintf("main echo: %+v", res)) // __AUTO_GENERATED_PRINT_VAR_END__
}
