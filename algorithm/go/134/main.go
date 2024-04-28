package main

import (
	"fmt"
)

func main() {
	// res := canCompleteCircuit([]int{5, 8, 2, 8}, []int{6, 5, 6, 6})
	res := canCompleteCircuit([]int{1, 2, 3, 4, 5}, []int{3, 4, 5, 1, 2})
	fmt.Println("res:", res)
}

func canCompleteCircuit(gas []int, cost []int) int {
	startIndex := -1
	count, cur_count := 0, 0

	for i := 0; i < len(gas); i++ {
		diff := (gas[i] - cost[i])
		count += diff
		cur_count += diff
		if startIndex == -1 {
			startIndex = i
		}
		if cur_count < 0 {
			cur_count = 0
			startIndex = -1
		}
	}

	if count >= 0 {
		return startIndex
	}
	return -1
}
