package main

import (
	"fmt"
	"math"
)

func main() {
	result := numSquares(12)
	fmt.Println("result:", result)
}

func numSquares(n int) int {
	dp := make([]int, n+1)

	for cap := 1; cap < len(dp); cap++ {
		minx := math.MaxInt
		for n := 1; n*n <= cap; n++ {
			minx = min(minx, dp[cap-(n*n)]+1)
			dp[cap] = int(minx)
		}
	}

	return dp[n]
}
