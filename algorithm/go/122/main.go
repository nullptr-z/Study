package main

import "fmt"

func main() {
	ret := maxProfit([]int{7, 1, 5, 3, 6, 4})
	fmt.Println("ret:", ret)
}

func maxProfit(prices []int) int {
	sum := 0

	for i := 1; i < len(prices); i++ {
		if prices[i] > prices[i-1] {
			sum += prices[i] - prices[i-1]
		}
	}

	return sum
}
