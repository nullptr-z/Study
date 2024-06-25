package main

import (
	"fmt"
)

func main() {
	ret := coinChange([]int{1, 2, 5}, 11)
	fmt.Println("ret:", ret)
}

func coinChange(coins []int, amount int) int {
	dp := make([]int, amount+1)
	for i := 1; i < len(dp); i++ {
		dp[i] = amount + 1
	}

	for _, n := range coins {
		for cap := n; cap < len(dp); cap++ {
			dp[cap] = min(dp[cap], dp[cap-n]+1)
		}
	}

	if dp[amount] > amount {
		return -1
	}

	return dp[amount]
}
