package main

import "fmt"

func removeElement(nums []int, val int) int {
	len := len(nums)
	if len == 0 {
		return 0
	}
	l, r := 0, len-1

	for l <= r {
		if nums[r] == val {
			r--
			continue
		}
		if nums[l] == val {
			nums[l] = nums[r]
			r--
		}
		l++
	}

	return l
}

func main() {
	ret := removeElement([]int{}, 3)
	fmt.Println("ret:", ret)
}
