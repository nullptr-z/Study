package main

func main() {
	findSubsequences([]int{4, 6, 7, 7})
}

func findSubsequences(nums []int) [][]int {
	result := make([][]int, 0)
	temp := make([]int, 0)
	backtrack(nums, temp, &result)

	return result
}

func backtrack(nums, temp []int, result *[][]int) {
	if len(temp) >= 2 {
		*result = append(*result, append([]int{}, temp...))
	}
	used := make(map[int]bool, len(nums))
	for i, n := range nums {
		if used[n] || len(temp) > 0 && n < temp[len(temp)-1] {
			continue
		}
		if !used[n] {
			used[n] = true
		}
		temp = append(temp, n)
		backtrack(nums[i+1:], temp, result)
		temp = temp[:len(temp)-1]
		used[n] = true
	}
}
