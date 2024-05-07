package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rightSideView(root *TreeNode) []int {
	var result []int
	var levels []int
	var queue = []*TreeNode{root}
	var count int = 1

	for len(queue) > 0 {
		node := queue[0]
		queue = queue[1:]
		count--

		if node != nil {
			queue = append(queue, node.Left, node.Right)
			levels = append(levels, node.Val)
		}

		if count == 0 {
			count = len(queue)
			if len(levels) > 0 {
				result = append(result, levels[len(levels)-1])
			}
			levels = []int{}
		}
	}

	return result
}
