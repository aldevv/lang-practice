package main

import (
	"fmt"
	"pack/pack1"
	"pack/pack2"
)

type Responser interface {
	Send() (*pack1.Response, error)
}

type Response struct {
	Message string
	Code    int
}

func Handle(r Responser) {
	res, _ := r.Send()
	fmt.Println(fmt.Sprintf("res: %+v", res))
}

func main() {
	mp1 := &pack1.MyPack1{}
	mp2 := &pack2.MyPack2{}

	Handle(mp1)
	Handle(mp2)
}
