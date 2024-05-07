package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func invertTree(root *TreeNode) *TreeNode {
	var stack = []*TreeNode{root}

	for len(stack) > 0 {
		end := len(stack) - 1
		top := stack[end]
		stack = stack[:end]

		if top == nil {
			continue
		}
		top.Left, top.Right = top.Right, top.Left
		stack = append(stack, top.Left, top.Right)
	}

	return root
}
