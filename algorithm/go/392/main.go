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

// 给定字符串 s 和 t ，判断 s 是否为 t 的子序列。

// 字符串的一个子序列是原始字符串删除一些（也可以不删除）字符而不改变剩余字符相对位置形成的新字符串。（例如，"ace"是"abcde"的一个子序列，而"aec"不是）。

// 示例 1：

// 输入：s = "abc", t = "ahbgdc"
// 输出：true
// 示例 2：

// 输入：s = "axc", t = "ahbgdc"
// 输出：false
// 提示：

// 0 <= s.length <= 100
// 0 <= t.length <= 10^4
// 两个字符串都只由小写字符组成。

// #
