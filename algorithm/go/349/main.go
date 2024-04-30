package main

import "fmt"

func main() {
	res := intersection([]int{1, 2, 2, 1}, []int{2, 2})
	fmt.Println("res:", res)
}

func intersection(nums1 []int, nums2 []int) []int {
	numsMap := make(map[int]bool, len(nums1))
	retArr := make([]int, 0)

	for _, n := range nums1 {
		numsMap[n] = true
	}

	for _, n := range nums2 {
		if numsMap[n] {
			retArr = append(retArr, n)
			numsMap[n] = false
			// delete(numsMap, n)
		}
	}

	return retArr
}
