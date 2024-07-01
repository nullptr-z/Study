package main

import (
	"fmt"
	"sort"
)

func main() {
	ret := lengthOfLIS([]int{10, 9, 2, 5, 3, 7, 101, 18})
	fmt.Println("ret:", ret)
}

func lengthOfLIS(nums []int) int {
	dp := make([]int, 0)
	for _, n := range nums {
		idx := sort.Search(len(dp), func(i int) bool {
			return dp[i] >= n
		})
		if idx < len(dp) {
			dp[idx] = n
		} else {
			dp = append(dp, n)
		}
	}

	return len(dp)
}
