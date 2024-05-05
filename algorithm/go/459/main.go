package main

import (
	"fmt"
	"strings"
)

func main() {
	res := repeatedSubstringPattern("abab")
	fmt.Println("res:", res)
}

func repeatedSubstringPattern(s string) bool {
	ss := s + s
	return strings.Contains(ss[1:len(ss)-1], s)
}
