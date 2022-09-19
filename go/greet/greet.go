package greet

import (
	"fmt"
)

func Hi(name string) {
	if len(name) == 0 {
		fmt.Println("Hi!")
		return
	}
	fmt.Printf("Hi, %v, nice to meet you!\n", name)
	return
}

func Buenas(name string) {
	if len(name) == 0 {
		fmt.Println("Hi!")
		return
	}
	fmt.Printf("Hola, %v, un gusto conocerte!\n", name)
	return
}
