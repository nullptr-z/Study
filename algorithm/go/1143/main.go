package main

import "fmt"

func main() {
	result := longestCommonSubsequence("abcde", "ace")
	fmt.Println("result:", result)
}

func longestCommonSubsequence(text1 string, text2 string) int {
	m, n := len(text1)+1, len(text2)+1
	dp := make([][]int, m+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, n+1)
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			if text1[i-1] == text2[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
			} else {
				dp[i][j] = max(dp[i][j-1], dp[i-1][j])
			}
		}
	}

	return dp[m-1][n-1]
}
