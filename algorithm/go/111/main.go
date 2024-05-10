package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minDepth(root *TreeNode) int {
	if root != nil {
		l := minDepth(root.Left) + 1
		r := minDepth(root.Right) + 1
		// 注意，如果都是空的就是叶子节点，返回谁都可以，因为都是l,r 0
		if root.Left == nil {
			return r
		}
		if root.Right == nil {
			return l
		}
		return min(l, r)
	}
	return 0
}
