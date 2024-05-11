package main

import (
	"fmt"
	"strconv"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func binaryTreePaths(root *TreeNode) []string {
	var result = new([]string)
	concussion(root, "", result)

	return *result
}

func concussion(root *TreeNode, preStr string, ss *[]string) {
	if root == nil {
		return
	}

	newStr := fmt.Sprint(preStr, "->", strconv.Itoa(root.Val))
	if preStr == "" {
		newStr = strconv.Itoa(root.Val)
	}

	if root.Left == nil && root.Right == nil {
		*ss = append(*ss, newStr)
		return
	}

	concussion(root.Left, newStr, ss)
	concussion(root.Right, newStr, ss)
}
