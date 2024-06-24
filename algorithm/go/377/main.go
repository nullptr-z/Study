package main

func main() {
	combinationSum4([]int{1, 2, 3}, 4)
}

func combinationSum4(nums []int, target int) int {
	dp := make([]int, target+1)
	dp[0] = 1

	for cap := 1; cap < len(dp); cap++ {
		for _, n := range nums {
			if cap >= n {
				dp[cap] += dp[cap-n]
			}
		}
	}

	return dp[len(dp)-1]
}
