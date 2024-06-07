package main

import "fmt"

func main() {
	ret := climbStairs(3)
	fmt.Println("ret:", ret)
}

func climbStairs(n int) int {
	pd := make([]int, n+1)
	pd[0] = 1
	pd[1] = 1
	for i := 2; i < n+1; i++ {
		pd[i] = pd[i-1] + pd[i-2]
	}

	return pd[len(pd)-1]
}
