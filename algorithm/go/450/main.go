package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func deleteNode(root *TreeNode, key int) *TreeNode {
	if root == nil {
		return nil
	}

	if root.Val == key {
		cur := root.Left
		if cur == nil {
			return root.Right
		}
		for cur.Right != nil {
			cur = cur.Right
		}
		cur.Right = root.Right
		return root.Left
	}

	root.Left = deleteNode(root.Left, key)
	root.Right = deleteNode(root.Right, key)

	return root
}
