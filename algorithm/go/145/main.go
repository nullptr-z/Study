package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func postorderTraversal(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	left := postorderTraversal(root.Left)
	right := postorderTraversal(root.Right)

	array := append(left, right...)
	array = append(array, root.Val)
	return array
}
