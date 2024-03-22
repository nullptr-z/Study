package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func postorderTraversals(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	left := postorderTraversals(root.Left)
	right := postorderTraversals(root.Right)

	array := append(left, right...)
	array = append(array, root.Val)
	return array
}

func postorderTraversal(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	stack := []*TreeNode{root}
	result := make([]int, 0)

	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if top != nil {
			result = append([]int{top.Val}, result...)
			stack = append(stack, top.Left, top.Right)
		}
	}

	return result
}
