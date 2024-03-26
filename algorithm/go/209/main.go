package main

import "math"

func minSubArrayLen(target int, nums []int) int {
	sum := nums[0]
	minLen := math.MaxInt
	lp, rp := 0, 1
	for lp < rp {
		if sum < target && rp < len(nums) {
			sum += nums[rp]
			rp++
			continue
		}
		if sum >= target {
			minLen = min(minLen, rp-lp)
			sum -= nums[lp]
		}
		lp++
	}

	if minLen == math.MaxInt {
		return 0
	}

	return minLen
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func main() {

}
