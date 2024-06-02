package main

import (
	"slices"
)

func main() {
	subsetsWithDup([]int{1, 2, 2})
}

func subsetsWithDup(nums []int) [][]int {
	result := make([][]int, 0)
	temp := make([]int, 0)
	slices.Sort(nums)
	backtrack(nums, 0, temp, &result)
	return result
}

func backtrack(nums []int, start int, temp []int, result *[][]int) {
	*result = append(*result, append([]int{}, temp...))

	first_usee := false
	for i := start; i < len(nums); i++ {
		// 垂直链路，这里一定是使用first_usee来判断，第一次使用一个值作为起始元素
		// 使用 i>0会破坏横向链路
		if first_usee && nums[i] == nums[i-1] {
			continue
		}
		temp = append(temp, nums[i])
		backtrack(nums, i+1, temp, result)
		first_usee = true
		temp = temp[:len(temp)-1]
	}
}
