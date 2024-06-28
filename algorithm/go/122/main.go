package main

import "fmt"

func main() {
	ret := maxProfit2([]int{7, 1, 5, 3, 6, 4})
	fmt.Println("ret:", ret)
}

func maxProfit(prices []int) int {
	sum := 0

	for i := 1; i < len(prices); i++ {
		if prices[i] > prices[i-1] {
			sum += prices[i] - prices[i-1]
		}
	}

	return sum
}

func maxProfit2(prices []int) int {
	dp := []int{-prices[0], 0}

	for _, p := range prices {
		dp[0] = max(dp[0], dp[1]-p)
		dp[1] = max(dp[1], dp[0]+p)
	}

	return dp[1]
}
