package main

import "math"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isBalanced(root *TreeNode) bool {
	if root == nil {
		return true
	}
	return concussion(root) != -1
}

func concussion(root *TreeNode) int {
	if root == nil {
		return 0
	}
	l := concussion(root.Left)
	r := concussion(root.Right)
	if math.Abs(float64(l)-float64(r)) > 1 || r == -1 || l == -1 {
		return -1
	}
	return max(l, r) + 1
}
