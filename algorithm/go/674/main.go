package main

func main() {
	findLengthOfLCIS([]int{1, 3, 5, 4, 7})
}

func findLengthOfLCIS(nums []int) int {
	maxLen := 1
	curLen := 1
	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[i-1] {
			curLen++
		} else {
			curLen = 1
		}
		if curLen > maxLen {
			maxLen = curLen
		}
	}

	return maxLen
}
