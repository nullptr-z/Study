package main

import (
	"fmt"
)

func main() {
	// ret := lastStoneWeightII([]int{2, 7, 4, 1, 8, 1})
	ret := lastStoneWeightII([]int{31, 26, 33, 21, 40})
	fmt.Println("ret:", ret)
}

func lastStoneWeightII(stones []int) int {
	sum := 0
	for _, n := range stones {
		sum += n
	}

	pack := sum / 2
	dp := make([]int, pack+1)
	for _, n := range stones {
		for i := pack; i >= n; i-- {
			dp[i] = max(dp[i], dp[i-n]+n)
		}
	}

	return sum - dp[pack] - dp[pack]
}
