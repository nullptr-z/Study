package main

type TreeNode struct {
	Left  *TreeNode
	Right *TreeNode
	Val   int
}

func main() {
	root := genBinaryTree([]int{3, 2, 3, -1, 3, -1, 1})
	rob(root)
}

func rob(root *TreeNode) int {
	result0, result1 := backtrack(root)
	return max(result0, result1)
}

func backtrack(root *TreeNode) (int, int) {
	if root == nil {
		return 0, 0
	}
	left0, left1 := backtrack(root.Left)
	right0, right1 := backtrack(root.Right)

	steal := root.Val + left1 + right1
	no_steal := max(left0, left1) + max(right0, right1)

	return steal, no_steal
}

func genBinaryTree(arr []int) *TreeNode {
	var root *TreeNode = new(TreeNode)
	root.Val = arr[0]

	for i := 1; i < len(arr); i++ {
		if arr[i] != -1 {
			root.Left = &TreeNode{Val: arr[i]}
		}
		i++

		if arr[i] != -1 {
			root.Right = &TreeNode{Val: arr[i]}
		}
	}

	return root
}
