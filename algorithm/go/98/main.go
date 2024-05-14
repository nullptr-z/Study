package main

import (
	"math"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isValidBST(root *TreeNode) bool {
	preVal := math.MinInt64
	return recursion(root, &preVal)
}

func recursion(root *TreeNode, preVal *int) bool {
	if root == nil {
		return true
	}
	if !recursion(root.Left, preVal) || root.Val <= *preVal {
		return false
	}
	*preVal = root.Val
	return recursion(root.Right, preVal)
}
