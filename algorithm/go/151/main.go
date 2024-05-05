package main

import (
	"fmt"
)

func main() {
	res := reverseWords("the sky is blue")
	fmt.Println("res:", res)
}

func reverseWords(s string) string {
	var result string
	var sStack []string

	start := -1
	for i, c := range s {
		if start == -1 && c != ' ' {
			start = i
		}
		isEnd := i == len(s)-1
		if (c == ' ' || isEnd) && start != -1 {
			if isEnd && c != ' ' {
				i += 1
			}
			sStack = append(sStack, s[start:i])
			start = -1
		}
	}

	for i := len(sStack) - 1; i >= 0; i-- {
		result += sStack[i]
		if i != 0 {
			result += " "
		}
	}

	return result
}
