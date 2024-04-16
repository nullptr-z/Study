package main

import (
	"fmt"
	"sort"
)

func main() {
	ret := findContentChildren([]int{4, 1, 2, 3}, []int{1, 2})
	fmt.Println("ret:", ret)
}

func findContentChildren(g []int, s []int) int {
	sort.Ints(g)
	sort.Ints(s)

	count := 0
	sIdx, sLen := 0, len(s)

	for _, child := range g {
		for sIdx < sLen {
			if child <= s[sIdx] {
				count++
				sIdx++
				break
			}
			sIdx++
		}
	}

	return count
}
