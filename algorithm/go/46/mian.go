package main

func main() {
	permute([]int{1, 2, 3})
}

func permute(nums []int) [][]int {
	result := make([][]int, 0)
	temp := make([]int, 0)
	used := make([]bool, len(nums))
	backtrack(nums, temp, &used, &result)

	return result
}

func backtrack(nums, temp []int, used *[]bool, result *[][]int) {
	if len(temp) == len(nums) {
		*result = append(*result, append([]int{}, temp...))
		return
	}

	for i := 0; i < len(nums); i++ {
		if (*used)[i] {
			continue
		}
		temp = append(temp, nums[i])
		(*used)[i] = true
		backtrack(nums, temp, used, result)
		(*used)[i] = false
		temp = temp[:len(temp)-1]
	}
}
