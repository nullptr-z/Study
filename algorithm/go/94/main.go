package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func inorderTraversals(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}
	left := inorderTraversal(root.Left)
	right := inorderTraversal(root.Right)

	array := append(left, root.Val)
	array = append(array, right...)
	return array
}

func inorderTraversal(root *TreeNode) []int {
	if root == nil {
		return []int{}
	}

	stack := []*TreeNode{}
	result := []int{}
	currentNode := root

	for currentNode != nil || len(stack) > 0 {
		// Traverse to the leftmost node
		for currentNode != nil {
			stack = append(stack, currentNode)
			currentNode = currentNode.Left
		}

		// pop stack top ndoe
		currentNode = stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		// Visit the popped node
		result = append(result, currentNode.Val)

		// Move to the right subtree
		currentNode = currentNode.Right
	}

	return result
}
