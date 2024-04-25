package main

import (
	"fmt"
	"math"
	"slices"
)

func main() {
	ret := largestSumAfterKNegations([]int{-8, 3, -5, -3, -5, -2}, 6)
	fmt.Println("ret:", ret)
}

func largestSumAfterKNegations(nums []int, k int) int {
	slices.SortFunc(nums, func(a int, b int) int {
		diff := int(math.Abs(float64(b))) - int(math.Abs(float64(a)))
		if b+a == 0 {
			return -1
		}
		return diff
	})

	for i, v := range nums {
		if k <= 0 {
			break
		}
		// 主要为了处理，排序后，在正数和正数之间的负数
		if v < 0 {
			k--
			nums[i] = 0 - nums[i]
		}
	}
	fmt.Println("nums:", nums)

	if k%2 == 1 {
		end := len(nums) - 1
		nums[end] = 0 - nums[end]
	}

	sum := 0
	for _, num := range nums {
		sum += num
	}

	return sum
}
