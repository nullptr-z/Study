package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	stack := make([]*TreeNode, 0)
	stack = append(stack, root)

	for len(stack) > 0 {
		end := len(stack) - 1
		top := stack[end]
		stack = stack[:end]
		if top == nil {
			continue
		}

		if top.Val > p.Val && top.Val > q.Val {
			stack = append(stack, top.Left)
		} else if top.Val < p.Val && top.Val < q.Val {
			stack = append(stack, top.Right)
		} else {
			return top
		}
	}

	return nil
}
