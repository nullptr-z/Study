package main

import "fmt"

func main() {
	result := maxUncrossedLines([]int{1, 4, 2}, []int{1, 2, 4})
	fmt.Println("result:", result)
}

func maxUncrossedLines(nums1 []int, nums2 []int) int {
	m, n := len(nums1)+1, len(nums2)+1
	dp := make([][]int, m)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, n)
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			if nums1[i-1] == nums2[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
				continue
			}
			dp[i][j] = max(dp[i-1][j], dp[i][j-1])
		}
	}

	return dp[m-1][n-1]
}
