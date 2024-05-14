package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func constructMaximumBinaryTree(nums []int) *TreeNode {
	if len(nums) < 1 {
		return nil
	}
	maxIdx, maxVal := 0, 0
	for i := 0; i < len(nums); i++ {
		if nums[i] > maxVal {
			maxIdx = i
			maxVal = nums[i]
		}
	}

	root := &TreeNode{Val: maxVal}
	root.Left = constructMaximumBinaryTree(nums[:maxIdx])
	root.Right = constructMaximumBinaryTree(nums[maxIdx+1:])

	return root
}
