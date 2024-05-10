package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 中序行不通
func isSymmetric(root *TreeNode) bool {
	return concussion(root.Left, root.Right)
}

func concussion(left *TreeNode, right *TreeNode) bool {
	if left == right {
		return true
	}
	if left == nil || right == nil || (left.Val != right.Val) {
		return false
	}

	l_ret := concussion(left.Left, right.Right)
	r_ret := concussion(left.Right, right.Left)
	return l_ret && r_ret
}
