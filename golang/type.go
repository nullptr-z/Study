package main

import (
	"fmt"
)

func main() {

	var info Info
	info.name = "小明"
	info.age = 23
	fmt.Printf("%#v", info)
}

type Name struct {
	name string
}

type Age struct {
	Name
	age int
}

type Info struct {
	Age
}
