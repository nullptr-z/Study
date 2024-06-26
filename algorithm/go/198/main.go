package main

import "fmt"

func main() {
	// ret := rob([]int{1, 2, 3, 1})
	ret := rob([]int{2, 1, 1, 2})
	fmt.Println("ret:", ret)
}

func rob(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}

	if len(nums) == 2 {
		return max(nums[0], nums[1])
	}

	dp := make([]int, len(nums))
	dp[0] = nums[0]
	dp[1] = max(nums[0], nums[1])

	for i := 2; i < len(dp); i++ {
		dp[i] = max(dp[i-1], dp[i-2]+nums[i])
	}

	return dp[len(dp)-1]
}
