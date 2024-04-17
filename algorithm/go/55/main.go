package main

import "fmt"

func main() {
	ret := canJump([]int{3, 2, 1, 0, 4})
	fmt.Println("ret:", ret)
}

func canJump(nums []int) bool {
	step := 0

	for i := 0; i < len(nums)-1; i++ {
		step = max(step, nums[i])
		step-- // 🎺当前步数，通往下一个位置的资格
		if step < 0 {
			return false
		}
	}

	return step >= 0
}
