package pack2

import "fmt"

type Response struct {
	Message string
	Code    int
}

type MyPack2 struct{}

func (mp2 MyPack2) Send() (*Response, error) {
	return &Response{}, nil
}

func Pack2() {
	fmt.Println("from pack2")
}
