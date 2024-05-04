package main

import (
	"fmt"
	"sort"
)

func main() {
	ret := threeSum([]int{0, 0, 0, 0})
	// ret := threeSum([]int{-1, 0, 1, 2, -1, -4})
	fmt.Println("ret:", ret)
}

func threeSum(nums []int) [][]int {
	var result [][]int
	var len = len(nums)

	sort.Ints(nums)

	for i := 0; i < len-2; i++ {
		if nums[i] > 0 {
			break
		}

		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		l := i + 1
		r := len - 1
		for l < r {
			sum := nums[l] + nums[r] + nums[i]
			if sum == 0 {
				result = append(result, []int{nums[i], nums[l], nums[r]})
			}

			if sum > 0 {
				r--
				for r > 0 && nums[r] == nums[r+1] {
					r--
				}
			} else {
				l++
				for l < len && nums[l] == nums[l-1] {
					l++
				}
			}
		}
	}

	return result
}
