package main

import (
	"fmt"
	"sort"
)

func main() {
	// source := [][]string{{"JFK", "AAA"}, {"AAA", "JFK"}, {"JFK", "CCC"}, {"JFK", "BBB"}, {"CCC", "JFK"}}
	source := [][]string{{"JFK", "KUL"}, {"JFK", "NRT"}, {"KUL", "JFK"}}

	findItinerary(source)
}

func findItinerary(tickets [][]string) []string {
	result := make([]string, 0)
	table := make(map[string][]string)

	// 构建邻接表
	for _, t := range tickets {
		key, value := t[0], t[1]
		table[key] = append(table[key], value)
	}

	// 将每个出发地的目的地进行排序
	for key := range table {
		sort.Strings(table[key])
	}

	backtrack("JFK", &table, &result)
	reverse(result)

	return result
}

func backtrack(departure string, table *map[string][]string, result *[]string) {
	for len((*table)[departure]) > 0 {
		// 删除第一个元素，然后递归
		arrive := (*table)[departure][0]
		(*table)[departure] = (*table)[departure][1:]
		backtrack(arrive, table, result)
	}
	fmt.Println("departure:", departure)
	*result = append(*result, departure)
}

func reverse(arr []string) {
	// 使用双指针法交换元素
	left, right := 0, len(arr)-1
	for left < right {
		arr[left], arr[right] = arr[right], arr[left]
		left++
		right--
	}
}
