package main

import "fmt"

func main() {
	subsets([]int{1, 2, 3})
}

func subsets(nums []int) [][]int {
	result := make([][]int, 0)
	temp := make([]int, 0)
	backtrack(nums, 0, temp, &result)
	fmt.Println("result:", result)
	return result
}

func backtrack(nums []int, start int, temp []int, result *[][]int) {
	if len(nums) == 0 {
		return
	}
	*result = append(*result, append([]int{}, temp...))

	for i := start; i < len(nums); i++ {
		temp = append(temp, nums[i])
		backtrack(nums, i+1, temp, result)
		temp = temp[:len(temp)-1]
	}
}
