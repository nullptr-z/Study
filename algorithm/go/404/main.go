package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sumOfLeftLeaves(root *TreeNode) int {
	return concussion(root, false)
}

func concussion(root *TreeNode, isLeft bool) int {
	if root == nil {
		return 0
	}
	if root.Left == nil && root.Right == nil {
		if isLeft {
			return root.Val
		} else {
			return 0
		}
	}
	l := concussion(root.Left, true)
	r := concussion(root.Right, false)
	return l + r
}
