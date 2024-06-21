// 给你一个非负整数数组 nums 和一个整数 target 。

// 向数组中的每个整数前添加 '+' 或 '-' ，然后串联起所有整数，可以构造一个 表达式 ：

// 例如，nums = [2, 1] ，可以在 2 之前添加 '+' ，在 1 之前添加 '-' ，然后串联起来得到表达式 "+2-1" 。
// 返回可以通过上述方法构造的、运算结果等于 target 的不同 表达式 的数目。

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
