package main

import (
	"fmt"
	"sort"
)

func main() {
	// ret := fourSum([]int{1, 0, -1, 0, -2, 2}, 0)
	// ret := fourSum([]int{1, -2, -5, -4, -3, 3, 3, 5}, -11)
	ret := fourSum([]int{0, 0, 0, 0}, 0)
	fmt.Println("ret:", ret)
}

func fourSum(nums []int, target int) [][]int {
	var result [][]int
	var len = len(nums)
	if len < 4 {
		return result
	}

	sort.Ints(nums)

	for i := 0; i < len-3; i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		if nums[i] >= 0 && nums[i] > target {
			break
		}

		num2_idx := i + 1
		for j := num2_idx; j < len-2; j++ {
			if j > num2_idx && nums[j] == nums[j-1] {
				continue
			}

			pre2sum := nums[j] + nums[i]
			if pre2sum >= 0 && pre2sum > target {
				break
			}

			l, r := j+1, len-1
			for l < r {
				sum := nums[l] + nums[r] + pre2sum
				if sum == target {
					result = append(result, []int{nums[i], nums[j], nums[l], nums[r]})
				}

				if sum > target {
					r--
					for r > l && nums[r] == nums[r+1] {
						r--
					}
				} else {
					l++
					for l < r && nums[l] == nums[l-1] {
						l++
					}
				}
			}
		}
	}

	return result
}
