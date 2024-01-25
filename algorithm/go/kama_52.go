package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func completeBackpack(wvs [][]int, n int) int {
	dp := make([]int, n+1)
	for _, wv := range wvs {
		for cap := 1; cap < len(dp); cap++ {
			remain := cap - wv[0]
			if remain >= 0 {
				dp[cap] = max(dp[cap], dp[remain]+wv[1])
			}
		}
	}

	return dp[len(dp)-1]
}

func main() {
	// 读取输入
	reader := bufio.NewReader(os.Stdin)
	input, _ := reader.ReadString('\n')
	input = strings.TrimSpace(input)
	nv := strings.Split(input, " ")
	N, _ := strconv.Atoi(nv[0])
	V, _ := strconv.Atoi(nv[1])

	// 读取研究材料的重量和价值
	var materials [][]int
	for i := 0; i < N; i++ {
		line, _ := reader.ReadString('\n')
		line = strings.TrimSpace(line)
		wv := strings.Split(line, " ")
		weight, _ := strconv.Atoi(wv[0])
		value, _ := strconv.Atoi(wv[1])
		materials = append(materials, []int{weight, value})
	}

	// 调用处理函数
	result := completeBackpack(materials, V)

	// 输出结果
	fmt.Println(result)
}
