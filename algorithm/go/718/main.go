package main

func main() {
	findLength([]int{1, 2, 3, 2, 1}, []int{3, 2, 1, 4, 7})
}

func findLength(nums1 []int, nums2 []int) int {
	m, n := len(nums1)+1, len(nums2)+1
	dp := make([][]int, m)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, n)
	}

	maxLen := 0
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			if nums1[i-1] == nums2[j-1] {
				dp[i][j] = dp[i-1][j-1] + 1
				maxLen = max(maxLen, dp[i][j])
			}
		}
	}

	return maxLen
}
