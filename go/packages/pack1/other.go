package pack1

import "fmt"

type Response struct {
	Message string
	Code    int
}

type MyPack1 struct{}

func (mp1 MyPack1) Send() (*Response, error) {
	return &Response{}, nil
}

func Pack1() {
	fmt.Println("from pack1")
}
