package main

import (
	"flag"
	"fmt"
	"strings"
)

var sep = flag.String("s", "^", "flag.String")

func main() {
	flag.Parse()
	fmt.Printf(strings.Join(flag.Args(), *sep))
}
