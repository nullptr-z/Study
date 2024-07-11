package main

import "fmt"

func main() {
	result := countSubstrings("abac")
	fmt.Println("result:", result)
}

func countSubstrings(s string) int {
	len := len(s)
	dp := make([][]bool, len)
	for i := 0; i < len; i++ {
		dp[i] = make([]bool, len)
	}
	count := 0

	for i := len; i >= 0; i-- {
		for j := i; j < len; j++ {
			if s[i] == s[j] && (j-i <= 1 || dp[i+1][j-1]) {
				dp[i][j] = true
				count++
			}
		}
	}

	return count
}
