package main

import "fmt"

func main() {
	res := candy([]int{1, 3, 2, 2, 1})
	fmt.Println("res:", res)
}

func candy(ratings []int) int {
	len := len(ratings)
	sugars := make([]int, len)
	for i := 0; i < len; i++ {
		sugars[i] = 1
	}

	for i := 1; i < len; i++ {
		if ratings[i] > ratings[i-1] {
			sugars[i] = sugars[i-1] + 1
		}
	}

	for i := len - 1; i > 0; i-- {
		if ratings[i-1] > ratings[i] {
			sugars[i-1] = max(sugars[i-1], sugars[i]+1)
		}
	}

	sum := 0
	for _, s := range sugars {
		sum += s
	}

	return sum
}
