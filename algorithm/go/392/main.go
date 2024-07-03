package main

import "fmt"

func main() {
	ret := isSubsequence("abc", "ahbgdc")
	fmt.Println("ret:", ret)
}

func isSubsequence(s string, t string) bool {
	sLen := len(s)
	tLen := len(t)
	dp := make([][]int, sLen+1)

	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, tLen+1)
	}

	for i := 1; i <= sLen; i++ {
		for j := 1; j <= tLen; j++ {
			if s[i-1] == t[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
				continue
			}
			dp[i][j] = max(dp[i-1][j], dp[i][j-1])
		}
	}

	return dp[sLen][tLen] == sLen
}
