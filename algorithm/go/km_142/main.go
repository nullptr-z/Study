package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	line1, line2 := getStdin()

	s1 := []byte(line1)
	s2 := []byte(line2)

	s1Len := len(s1)
	s2Len := len(s2)

	dp := make([][]int, s1Len+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, s2Len+1)
	}

	for i := 1; i <= s1Len; i++ {
		dp[i][0] = dp[i-1][0] + int(s1[i-1])
	}
	for j := 1; j <= s2Len; j++ {
		dp[0][j] = dp[0][j-1] + int(s2[j-1])
	}

	for i := 1; i <= s1Len; i++ {
		for j := 1; j <= s2Len; j++ {
			if s1[i-1] == s2[j-1] {
				dp[i][j] = dp[i-1][j-1]
				continue
			}
			dp[i][j] = min(dp[i-1][j]+int(s1[i-1]), dp[i][j-1]+int(s2[j-1]))
		}
	}

	fmt.Println(dp[s1Len][s2Len])
}

func getStdin() (string, string) {
	reader := bufio.NewReader(os.Stdin)
	// 读取第一行数据
	line1, _ := reader.ReadString('\n')
	line1 = strings.TrimSpace(line1) // 去掉换行符
	// 读取第二行数据
	line2, _ := reader.ReadString('\n')
	line2 = strings.TrimSpace(line2) // 去掉换行符

	return line1, line2
}

func min(a int, b int) int {
	if a < b {
		return a
	}
	return b
}
