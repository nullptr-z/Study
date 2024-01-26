package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func climbStairs(n int, m int) int {
	dp := make([]int, n+1)
	dp[0] = 1

	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			if i-j >= 0 {
				dp[i] += dp[i-j]
			}
		}
	}

	return dp[n]
}

func main() {
	// 读取输入
	reader := bufio.NewReader(os.Stdin)
	input, _ := reader.ReadString('\n')
	input = strings.TrimSpace(input)
	nv := strings.Split(input, " ")
	n, _ := strconv.Atoi(nv[0])
	m, _ := strconv.Atoi(nv[1])

	result := climbStairs(n, m)
	fmt.Println(result)
}
