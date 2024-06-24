package main

import "fmt"

func main() {
	ret := findTargetSumWays([]int{1, 1, 1, 1, 1}, 3)
	fmt.Println("ret:", ret)
}

func findTargetSumWays(nums []int, target int) int {
	sum := 0
	for _, n := range nums {
		sum += n
	}

	diff := (sum - target)
	if target > sum || diff%2 == 1 {
		return 0
	}

	pack := diff / 2
	dp := make([]int, pack+1)
	dp[0] = 1

	for _, n := range nums {
		for cap := pack; cap >= n; cap-- {
			dp[cap] += dp[cap-n]
		}
	}

	return dp[len(dp)-1]
}
