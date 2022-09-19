package greet_test

import (
	"github.com/aldevv/lang-practice/greet"
)

func ExampleHi() {
	greet.Hi("")
	// Output:
	// Hi!
}

func ExampleHi_with_arguments() {
	greet.Hi("Rob")
	//Output:
	//Hi, Rob, nice to meet you!
}
