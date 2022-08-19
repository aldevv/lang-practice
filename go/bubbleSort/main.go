package main

import (
	"fmt"
)

func main() {
	a := []int{10, 9, 8, 7}
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(a)-1-i; j++ {
			tmp := a[j]
			a[j] = a[j+1]
			a[j+1] = tmp
		}
	}

	fmt.Println(a)
}
