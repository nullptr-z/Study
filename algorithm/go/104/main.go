package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func maxDepth(root *TreeNode) int {
	if root != nil {
		l := maxDepth(root.Left)
		r := maxDepth(root.Right)
		return max(l, r) + 1
	}
	return 0
}
