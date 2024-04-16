package main

import "fmt"

func main() {
	ret := wiggleMaxLength([]int{1, 7, 4, 9, 2, 5})
	fmt.Println("ret:", ret)
}

func wiggleMaxLength(nums []int) int {
	if len(nums) < 2 {
		return len(nums)
	}

	status, counter := 0, 1

	for i := 1; i < len(nums); i++ {
		diff := nums[i] - nums[i-1]
		if diff > 0 && status <= 0 {
			counter++
			status = diff
		}
		if diff < 0 && status >= 0 {
			counter++
			status = diff
		}
	}

	return counter
}
