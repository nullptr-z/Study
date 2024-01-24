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

func oneTwoBackpack(weight []int, price []int, n int) int {
	dp := make([]int, n+1)

	for i := 0; i < len(weight); i++ {
		for cap := n; cap >= 0; cap-- {
			remain := cap - weight[i]
			if remain >= 0 {
				dp[cap] = max(dp[cap], price[i]+dp[remain])
			}
		}
	}

	return dp[n]
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	inputText := scanner.Text()
	scanner.Scan()
	inputText1 := scanner.Text()
	scanner.Scan()
	inputText2 := scanner.Text()
	// fmt.Println(inputText+"\n", inputText1+"\n", inputText2)
	line := parse(inputText)
	// fmt.Println("line:", line)
	weight := parse(inputText1)
	// fmt.Println("weight:", weight)
	price := parse(inputText2)
	// fmt.Println("price:", price)
	result := oneTwoBackpack(weight, price, line[1])
	fmt.Println(result)
}

func parse(inputString string) []int {
	parts := strings.Fields(inputString)
	var intSlice []int

	// 遍历分割后的部分并将其转换为整数
	for _, part := range parts {
		// 使用 strconv.Atoi 将字符串转换为整数
		num, _ := strconv.Atoi(part)
		// 将整数添加到切片中
		intSlice = append(intSlice, num)
	}
	return intSlice
}
