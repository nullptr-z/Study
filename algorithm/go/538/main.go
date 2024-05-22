package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func convertBST(root *TreeNode) *TreeNode {
	var sum *int = new(int)
	return concussion(root, sum)
}

func concussion(root *TreeNode, sum *int) *TreeNode {
	if root == nil {
		return nil
	}

	concussion(root.Right, sum)
	*sum += root.Val
	root.Val = *sum
	concussion(root.Left, sum)

	return root
}
