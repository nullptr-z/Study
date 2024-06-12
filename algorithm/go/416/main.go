package main

import (
	"fmt"
)

func main() {
	ret := canPartition([]int{1, 2, 5})
	fmt.Println("ret:", ret)
}

func canPartition(nums []int) bool {
	sum := 0
	for _, n := range nums {
		sum += n
	}
	if sum%2 != 0 {
		return false
	}
	pack := sum / 2

	// 保留一个起始位置 0
	dp := make([]int, pack+1)
	for _, num := range nums {
		for cap := pack; cap >= num; cap-- {
			dp[cap] = max(dp[cap], dp[cap-num]+num)
			if dp[cap] == pack {
				return true
			}
		}
	}

	return false
}
