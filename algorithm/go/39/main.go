package main

import "fmt"

func main() {
	combinationSum([]int{2, 3, 5}, 8)
}

func combinationSum(candidates []int, target int) [][]int {
	result := make([][]int, 0)
	concussion(candidates, []int{}, &result, 0, target)
	fmt.Println("result:", result)
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
		cur := candidates[i]
		// i之后不会再出现 cur, 之后出现个数必然不同，因为candidates 元素不重复
		concussion(candidates[i:], append(temp, cur), result, sum+cur, target)
	}
}
