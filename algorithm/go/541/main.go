package main

import "fmt"

func main() {
	res := reverseStr("abcdefg", 2)
	fmt.Println("res:", res)
}

func reverseStr(s string, k int) string {
	len, bStr := len(s), []byte(s)
	k2 := 2 * k
	i := 0

	for {
		l := i
		r := l + k - 1
		if l >= len {
			break
		}

		if r >= len {
			// 剩余不足k，会进入这里
			r = len - 1
		}

		for l < r {
			bStr[l], bStr[r] = bStr[r], bStr[l]
			l++
			r--
		}

		i += k2
	}

	retStr := string(bStr)
	return retStr
}
