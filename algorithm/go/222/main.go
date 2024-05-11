package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func countNodes(root *TreeNode) int {
	stack := []*TreeNode{root}
	count := 0

	for len(stack) > 0 {
		top := stack[0]
		stack = stack[1:]
		if top == nil {
			continue
		}
		count++

		stack = append(stack, top.Left)
		stack = append(stack, top.Right)
	}

	return count
}
