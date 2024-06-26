package main

func main() {
	wordBreak("leetcode", []string{"leet", "code"})
}

func wordBreak(s string, wordDict []string) bool {
	dp := make([]bool, len(s)+1)
	dp[0] = true // 空字符总是满足条件
	used := make(map[string]bool)

	for _, word := range wordDict {
		used[word] = true
	}

	for i := 1; i < len(dp); i++ {
		for j := 0; j < i; j++ {
			if used[s[j:i]] && dp[j] {
				dp[i] = true
				break
			}
		}
	}

	return dp[len(dp)-1]
}
