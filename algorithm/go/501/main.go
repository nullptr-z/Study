package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findMode(root *TreeNode) []int {
	result := make([]int, 0)
	stack := make([]*TreeNode, 0)
	counter := make(map[int]int, 0)
	top := root
	maxCount := 0

	for len(stack) > 0 || top != nil {
		for top != nil {
			stack = append(stack, top)
			top = top.Left
		}

		end := len(stack) - 1
		top = stack[end]
		stack = stack[:end]

		count := counter[top.Val] + 1
		for len(result) > 0 {
			end := len(result) - 1
			maxVla := result[end]
			if count > counter[maxVla] {
				result = result[:len(result)-1]
			} else {
				maxCount = counter[maxVla]
				break
			}
		}
		counter[top.Val] = count
		if counter[top.Val] >= maxCount {
			result = append(result, top.Val)
		}

		top = top.Right
	}

	return result
}
