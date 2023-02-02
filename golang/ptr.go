package main

import (
	"fmt"
)

func main() {
	p := ptrTest1()
	fmt.Println("v:", *p)
}

func ptrTest1() *int {
	v := 1
	return &v
}
