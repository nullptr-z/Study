package main

import (
	"math"
)

func main() {
	jump([]int{2, 3, 1, 1, 4})
}

func jump(nums []int) int {
	end := len(nums) - 1
	jump := 0
	nextIndex := 0
	maxIdx := 0
	for i, step := range nums {
		if maxIdx >= end {
			break
		}
		nextIndex = int(math.Max(float64(nextIndex), float64(i+step)))
		if i == maxIdx {
			maxIdx = nextIndex
			jump++
		}
	}
	return jump
}
