package main

func main() {
	maxProfit([]int{7, 1, 5, 3, 6, 4})
}

func maxProfit(prices []int) int {
	// 我们需要始终唯一prices[0..i]之间的最小值(dp[0])
	// prices[0..i],随着 i 不停增加，出现的新的最大值，与最小值(dp[0])求差
	// dp[0]含义，prices[0..i]之间最小的那个数的相反数;
	// dp[1]含义， prices[0..i]之间最大收益，最终结果就是题解
	dp := []int{prices[0], 0} // [0..1]
	for _, p := range prices {
		dp[0] = min(dp[0], p)       // 维护最小值
		dp[1] = max(dp[1], p-dp[0]) // 求差值
	}

	return dp[1]
}
