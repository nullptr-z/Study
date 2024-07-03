package main

import "fmt"

func main() {
	ret := minDistance("leetcode", "etco")
	fmt.Println("ret:", ret)
}

func minDistance(word1 string, word2 string) int {
	m := len(word1)
	n := len(word2)
	dp := make([][]int, m+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, n+1)
	}

	for i, w1 := range word1 {
		for j, w2 := range word2 {
			if w1 == w2 {
				dp[i+1][j+1] = dp[i][j] + 1
				continue
			}
			dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1])
		}
	}

	return max(m-dp[m][n], 0) + max(n-dp[m][n], 0)
	// return (m + n) - (dp[m][n] << 1)
}
