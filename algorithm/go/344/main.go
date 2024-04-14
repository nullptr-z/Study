package main

import "fmt"

func main() {
	b := []byte{'h', 'e', 'l', 'l', 'o'}
	fmt.Println("b:", b)
	reverseString(b)
	fmt.Println("b:", b)

}

func reverseString(s []byte) {
	l, r := 0, len(s)-1
	for l < r {
		// s[l] ^= s[r]
		// s[r] ^= s[l]
		// s[l] ^= s[r]
		s[l], s[r] = s[r], s[l]
		l++
		r--
	}
}
