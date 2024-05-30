package main

import (
	"slices"
)

func main() {
	combinationSum2([]int{10, 1, 2, 7, 6, 1, 5}, 8)
}

func combinationSum2(candidates []int, target int) [][]int {
	result := make([][]int, 0)
	slices.Sort(candidates)
	concussion(candidates, []int{}, &result, 0, target)
	return result
}

func concussion(candidates, temp []int, result *[][]int, sum int, target int) {
	if sum >= target {
		if sum == target {
			*result = append(*result, append([]int{}, temp...))
		}
		return
	}

	for i := 0; i < len(candidates); i++ {
		if i > 0 && candidates[i] == candidates[i-1] {
			continue
		}
		cur := candidates[i]
		concussion(candidates[i+1:], append(temp, cur), result, sum+cur, target)
	}
}
