package main

import "math"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func getMinimumDifference(root *TreeNode) int {
	var preVal *int
	minDifference := math.MaxInt
	stack := make([]*TreeNode, 0)
	stack = append(stack, root)
	for len(stack) > 0 {
		last := len(stack) - 1
		top := stack[last]
		stack = stack[:last]
		if top == nil {
			continue
		}
		stack = append(stack, top.Right)
		top.Right = nil
		if top.Left != nil {
			stack = append(stack, top)
		} else {
			if preVal != nil {
				minDifference = min(minDifference, top.Val-*preVal)
			}
			preVal = &top.Val
		}
		stack = append(stack, top.Left)
		top.Left = nil
	}

	return minDifference
}

func main() {
	root := new(TreeNode)
	getMinimumDifference(root)
}
