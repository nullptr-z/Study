package main

func main() {
	minCostClimbingStairs([]int{1, 100, 1, 1, 1, 100, 1, 1, 100, 1})
	minCostClimbingStairs([]int{10, 15, 20})
}

func minCostClimbingStairs(cost []int) int {
	cost = append(cost, 0) // 追加一个顶楼（结果)
	dp := make([]int, len(cost))
	dp[0] = cost[0]
	dp[1] = cost[1]

	for i := 2; i < len(cost); i++ {
		dp[i] = min(dp[i-1]+cost[i], dp[i-2]+cost[i])
	}
	return dp[len(dp)-1]
}
