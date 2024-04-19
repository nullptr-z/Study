package main

import "fmt"

func main() {
	ret := twoSum([]int{2, 7, 11, 15}, 9)
	fmt.Println("ret:", ret)
}

func twoSum(nums []int, target int) []int {
	mp := make(map[int]int, len(nums))

	for i, val := range nums {
		if o, ok := mp[val]; ok {
			return []int{o, i}
		}
		mp[target-val] = i
	}

	return []int{}
}
