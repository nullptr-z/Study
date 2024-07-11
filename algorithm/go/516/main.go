package main

func main() {
	longestPalindromeSubseq("bbbab")
}

func longestPalindromeSubseq(s string) int {
	len := len(s)
	dp := make([][]int, len)
	for i := 0; i < len; i++ {
		dp[i] = make([]int, len)
	}

	for i := len - 1; i >= 0; i-- {
		dp[i][i] = 1
		for j := i + 1; j < len; j++ {
			if s[i] == s[j] {
				dp[i][j] = dp[i+1][j-1] + 2
				continue
			}
			dp[i][j] = max(dp[i+1][j], dp[i][j-1])
		}
	}

	return dp[0][len-1]
}
