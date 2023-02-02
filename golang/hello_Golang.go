package main

import (
	"fmt"
	"os"
	"bufio"
)

func main() {
	fmt.Println("hello_Golang")

	input := bufio.NewScanner(os.Stdin)

	for   input.Scan() {
		fmt.Println("input",input.Text())
	}

	for _, v := range os.Args[1:] {
		fmt.Println("v",v)
	}
}
