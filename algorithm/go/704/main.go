package main

import "fmt"

func main() {
	ret := search([]int{-1, 0, 3, 5, 9, 12}, 9)
	fmt.Println("ret:", ret)
}

func search(nums []int, target int) int {
	len := len(nums)
	lPtr, rPtr := 0, len

	for lPtr < rPtr {
		mid := (lPtr >> 1) + (rPtr >> 1)
		if nums[mid] > target {
			rPtr = mid
		} else if nums[mid] < target {
			lPtr = mid + 1
		} else {
			return mid
		}
	}

	return -1
}
