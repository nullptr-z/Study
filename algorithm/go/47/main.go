package main

func main() {
	permuteUnique([]int{1, 1, 2})
}

func permuteUnique(nums []int) [][]int {
	result := make([][]int, 0)
	temp := make([]int, 0)
	used := make(map[int]bool, 0)
	backtrack(nums, temp, &used, &result)

	return result
}

func backtrack(nums []int, temp []int, used *map[int]bool, result *[][]int) {
	if len(temp) == len(nums) {
		*result = append(*result, append([]int{}, temp...))
		return
	}

	flags := make(map[int]bool, 0)
	for i, n := range nums {
		if (*used)[i] || (flags[n]) {
			continue
		}
		flags[n] = true
		temp = append(temp, n)
		(*used)[i] = true
		backtrack(nums, temp, used, result)
		(*used)[i] = false
		temp = temp[:len(temp)-1]
	}
}
