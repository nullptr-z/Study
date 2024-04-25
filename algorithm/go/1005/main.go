package main

import (
	"fmt"
	"sort"
)

func main() {
	ret := largestSumAfterKNegations([]int{-8, 3, -5, -3, -5, -2}, 6)
	fmt.Println("ret:", ret)
}

func largestSumAfterKNegations(nums []int, k int) int {
	sort.Ints(nums)

	i := 0
	for i < len(nums) {
		if nums[i] >= 0 || k == 0 {
			break
		}
		nums[i] = 0 - nums[i]
		k--
		i++
	}
	i = min(i, len(nums)-1)

	if k%2 == 1 {
		if i > 0 && nums[i-1] < nums[i] {
			nums[i-1] = 0 - nums[i-1]
		} else {
			nums[i] = 0 - nums[i]
		}
	}

	sum := 0
	for _, num := range nums {
		sum += num
	}

	return sum
}
