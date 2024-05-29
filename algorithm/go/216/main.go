package main

func main() {
	combinationSum3(3, 7)
}

func combinationSum3(k int, n int) [][]int {
	combine := make([][]int, 0)
	items := make([]int, 0, k)
	sum := 0
	combination(k, n, sum, 1, items, &combine)
	return combine
}

func combination(k, n, sum, startIdx int, items []int, combine *[][]int) {
	if len(items) == k && sum == n {
		*combine = append(*combine, append([]int{}, items...))
		return
	}

	for i := startIdx; i <= 9; i++ {
		sum += i
		if sum > n {
			return
		}
		items = append(items, i)
		combination(k, n, sum, i+1, items, combine)
		sum -= i
		items = items[:len(items)-1]
	}
}
