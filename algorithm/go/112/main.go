package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func hasPathSum(root *TreeNode, targetSum int) bool {
	return concussion(root, targetSum, 0)
}

func concussion(root *TreeNode, targetSum, currentSum int) bool {
	if root == nil {
		return false
	}
	currentSum += root.Val
	if root.Left == nil && root.Right == nil {
		return targetSum == currentSum
	}
	l := concussion(root.Left, targetSum, currentSum)
	r := concussion(root.Right, targetSum, currentSum)

	return l || r
}
