package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func insertIntoBST(root *TreeNode, val int) *TreeNode {
	newNode := TreeNode{Val: val}
	if root == nil {
		return &newNode
	}

	// 不存在相等（==）的情况，只有大于小于
	if val < root.Val {
		if root.Left == nil {
			root.Left = &newNode
		} else {
			insertIntoBST(root.Left, val)
		}
	} else {
		if root.Right == nil {
			root.Right = &newNode
		} else {
			insertIntoBST(root.Right, val)
		}
	}

	return root
}
