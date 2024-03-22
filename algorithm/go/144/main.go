package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {
	stack := []*TreeNode{root}
	result := []int{}

	for len(stack) > 0 {
		last := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if last == nil {
			continue
		}
		result = append(result, last.Val)
		stack = append(stack, last.Right, last.Left)
	}

	return result
}

func preorderTraversal_recursion(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	left := preorderTraversal_recursion(root.Left)
	right := preorderTraversal_recursion(root.Right)
	array := []int{root.Val}
	array = append(array, left...)
	array = append(array, right...)
	return array
}
