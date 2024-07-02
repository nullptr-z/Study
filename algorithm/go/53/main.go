package main

import (
	"fmt"
)

func main() {
	ret := maxSubArray([]int{-2, 1, -3, 4, -1, 2, 1, -5, 4})
	fmt.Println("ret:", ret)
}

func maxSubArray(nums []int) int {
	maxSum, sum := nums[0], 0

	for _, v := range nums {
		sum += v
		maxSum = max(maxSum, sum)
		if sum < 0 {
			sum = 0
		}
	}
	return maxSum
}

func maxSubArray_dp(nums []int) int {
	dp := make([]int, len(nums))
	dp[0] = nums[0]
	maxVal := nums[0]

	for i := 1; i < len(nums); i++ {
		dp[i] = max(dp[i-1]+nums[i], nums[i])
		maxVal = max(maxVal, dp[i])
	}

	return maxVal
}
