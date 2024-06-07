package main

import (
	"fmt"
)

func main() {
	ret := fib(4)
	fmt.Println("ret:", ret)
}

func fib(n int) int {
	dp := make([]int, n+1)
	dp[1] = 1
	for i := 2; i < len(dp); i++ {
		dp[i] = dp[i-1] + dp[i-2]
	}

	return dp[n]
}
