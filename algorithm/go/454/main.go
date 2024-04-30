package main

import "fmt"

func main() {
	res := fourSumCount([]int{1, 2}, []int{-2, -1}, []int{-1, 2}, []int{0, 2})
	fmt.Println("res:", res)
}

func fourSumCount(nums1 []int, nums2 []int, nums3 []int, nums4 []int) int {
	result := 0
	len := len(nums1) * 2
	maps := make(map[int]int, len)

	for _, n1 := range nums1 {
		for _, n2 := range nums2 {
			sum := n1 + n2
			maps[sum]++
		}
	}

	for _, n3 := range nums3 {
		for _, n4 := range nums4 {
			sum := n3 + n4
			diff := 0 - sum
			if n, ok := maps[diff]; ok {
				// 两组的交集, 是·乘·关系，所有是+n
				result += n
			}
		}
	}

	return result
}
