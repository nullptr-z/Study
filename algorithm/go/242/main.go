package main

import "fmt"

func main() {
	res := isAnagram("abc", "abd")
	fmt.Println("res:", res)
}

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	sMap := make(map[rune]int, 0)
	for _, c := range s {
		sMap[c]++
	}

	for _, c := range t {
		if v, ok := sMap[c]; !ok || v < 1 {
			return false
		}
		sMap[c]--
	}
	// tMap[k] != sMap[k]

	return true
}
