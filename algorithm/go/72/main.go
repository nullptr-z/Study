package main

import "fmt"

func main() {
	result := minDistance("horse", "ros")
	fmt.Println("result:", result)
}

func minDistance(word1 string, word2 string) int {
	w1Len, w2Len := len(word1), len(word2)
	dp := make([][]int, w1Len+1)

	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, w2Len+1)
	}

	for i := 0; i <= w2Len; i++ {
		dp[0][i] = i
	}

	for i := 0; i <= w1Len; i++ {
		dp[i][0] = i
	}

	for i := 1; i <= w1Len; i++ {
		for j := 1; j <= w2Len; j++ {
			if word1[i-1] == word2[j-1] {
				dp[i][j] = dp[i-1][j-1]
				continue
			}
			dp[i][j] = min(min(dp[i-1][j], dp[i][j-1]), dp[i-1][j-1]) + 1
		}
	}

	return dp[w1Len][w2Len]
}
