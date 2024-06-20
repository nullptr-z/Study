package main

import "fmt"

func main() {
	ret := findMaxForm([]string{"10", "0001", "111001", "1", "0"}, 5, 3)
	fmt.Println("ret:", ret)
}

func findMaxForm(strs []string, m int, n int) int {
	dict := make(map[string][2]int)
	for _, s := range strs {
		dict[s] = getNumCount(s)
	}

	dp := make([][]int, m+1)
	for i := 0; i < m+1; i++ {
		dp[i] = make([]int, n+1)
	}

	for _, s := range strs {
		cur := dict[s]
		for z := m; z >= cur[0]; z-- {
			for o := n; o >= cur[1]; o-- {
				dp[z][o] = max(dp[z][o], dp[z-cur[0]][o-cur[1]]+1)
			}
		}
	}

	return dp[m][n]
}

func getNumCount(s string) [2]int {
	zero, one := 0, 0
	for _, c := range s {
		switch c {
		case '1':
			one++
			continue
		case '0':
			zero++
			continue
		default:
			continue
		}
	}

	return [2]int{zero, one}
}
