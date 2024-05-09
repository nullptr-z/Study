package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSymmetric(root *TreeNode) bool {

	vals := *new([]int)
	concurrent(root, &vals)
	fmt.Println("vals:", vals)
	len := len(vals)
	if len == 1 {
		return true
	} else if len == 0 {
		return true
	}
	r, l := 0, len-1

	for r < l {
		if vals[r] != vals[l] {
			return false
		}
		r++
		l--
	}

	return true
}

func concurrent(root *TreeNode, vals *[]int) {
	if root != nil {
		if root.Left == nil && root.Right == nil {
			*vals = append(*vals, root.Val)
			return
		}
		concurrent(root.Left, vals)
		*vals = append(*vals, root.Val)
		concurrent(root.Right, vals)
	} else {
		*vals = append(*vals, -1)
	}
}
