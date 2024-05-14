package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func searchBST(root *TreeNode, val int) *TreeNode {
	next := root
	for next != nil {
		if val == next.Val {
			return next
		}
		if val > next.Val {
			next = next.Right
		} else {
			next = next.Left
		}
	}

	return nil
}
