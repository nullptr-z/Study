package main

func main() {
	combine(4, 1)
}

func combine(n int, k int) [][]int {
	result := make([][]int, 0)
	subArray := make([]int, 0, k)

	// 从 1 开始
	concussion(n, k, 1, subArray, &result)

	return result
}

func concussion(n, k, startIdx int, subArray []int, result *[][]int) {
	if len(subArray) == k {
		subArrCopy := make([]int, k)
		copy(subArrCopy, subArray)
		*result = append(*result, subArrCopy)
		return
	}
	for i := startIdx; i <= n; i++ {
		subArray = append(subArray, i)
		concussion(n, k, i+1, subArray, result)
		subArray = subArray[:len(subArray)-1]
	}
}
