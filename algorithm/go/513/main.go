package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func findBottomLeftValue(root *TreeNode) int {
	_, val := concussion(root, 0)
	return val
}

func concussion(root *TreeNode, depth int) (int, int) {
	if root == nil {
		return depth, 0
	}
	depth++
	if root.Left == nil && root.Right == nil {
		return depth, root.Val
	}

	l_depth, lv := concussion(root.Left, depth)
	r_depth, rv := concussion(root.Right, depth)

	if l_depth >= r_depth {
		return l_depth, lv
	} else {
		return r_depth, rv
	}
}
