package main

import (
	"sort"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrderBottom(root *TreeNode) [][]int {
	var result [][]int
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
				result = append(result, levels)
			}
			levels = []int{}
		}
	}

	sort.SliceStable(result, func(i, j int) bool {
		return i > j
	})

	return result
}
