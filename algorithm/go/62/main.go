package main

func main() {
	uniquePaths(3, 7)
}

func uniquePaths(m int, n int) int {
	dp := make([]int, n)

	for j := 0; j < n; j++ {
		if dp[j] == 0 {
			dp[j] = 1
		}
	}

	dp[0] = 1
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			dp[j] = dp[j-1] + dp[j]
		}
	}

	return dp[len(dp)-1]
}
