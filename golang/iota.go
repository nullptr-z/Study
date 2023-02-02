package main

import "fmt"

func main() {
	const (
		a uint = 1 << iota
		b
		c
		d
	)

	fmt.Println("%d", a)
	fmt.Println("%d", b)
	fmt.Println("%d", c)
	fmt.Println("%d", d)

	arr1 := [2]int{1, 2}
	arr2 := [...]int{1, 2}

	fmt.Println(arr1 != arr2)
}

type Flags uint
