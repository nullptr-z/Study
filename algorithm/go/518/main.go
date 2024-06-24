package main

import "fmt"

func main() {
	ret := change(5, []int{1, 2, 5})
	fmt.Println("ret:", ret)
}

func change(amount int, coins []int) int {
	dp := make([]int, amount+1)
	dp[0] = 1

	for _, c := range coins {
		for i := c; i < len(dp); i++ {
			dp[i] += dp[i-c]
		}
	}

	return dp[len(dp)-1]
}
