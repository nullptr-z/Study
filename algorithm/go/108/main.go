package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func sortedArrayToBST(nums []int) *TreeNode {
	if len(nums) == 0 {
		return nil
	}

	mid := len(nums) / 2
	l := sortedArrayToBST(nums[:mid])
	r := sortedArrayToBST(nums[mid+1:])
	root := &TreeNode{Val: nums[mid], Left: l, Right: r}

	return root
}
