package main

import "fmt"

func main() {
	ret := rob([]int{2, 3, 2})
	fmt.Println("ret:", ret)
}

func rob(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	if len(nums) == 2 {
		return max(nums[0], nums[1])
	}

	return max(robs(nums[1:]), robs(nums[:len(nums)-1]))
}

func robs(nums []int) int {

	size := len(nums)
	dp := make([]int, size)
	dp[0] = nums[0]
	dp[1] = max(nums[0], nums[1])
	for i := 2; i < size; i++ {
		dp[i] = max(dp[i-1], dp[i-2]+nums[i])
	}

	return dp[len(dp)-1]
}
