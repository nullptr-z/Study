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
