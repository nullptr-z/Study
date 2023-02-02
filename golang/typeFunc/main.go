package main

import (
	"fmt"

	"./typeFunc"
)

func main() {
	const i int64 = 64
	j := typeFunc.OutType(i)
	fmt.Printf("%d", j)
}
