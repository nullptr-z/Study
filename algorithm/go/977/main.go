package main

func main() {

}

func sortedSquares(nums []int) []int {
	for i, v := range nums {
		nums[i] = v * v
	}

	l, r := 0, len(nums)-1
	ret := make([]int, len(nums))
	index := len(nums) - 1

	for l <= r {
		if nums[l] > nums[r] {
			ret[index] = nums[l]
			l++
		} else {
			ret[index] = nums[r]
			r--
		}
		index--
	}

	return ret
}
