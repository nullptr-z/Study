package main

import "fmt"

func main() {
	res := detectCycle(NewLinked([]int{3, 2, 0, -4}))
	res.IterLink()
}

func detectCycle(head *ListNode) *ListNode {
	maps := make(map[string]bool)
	cur := head

	for cur != nil {
		ads := fmt.Sprintf("%p", cur)
		if maps[ads] {
			return cur
		}
		maps[ads] = true
		cur = cur.Next
	}

	return cur
}

type ListNode struct {
	Val  int
	Next *ListNode
}

func NewLinked(nodes []int) *ListNode {
	var dummy = ListNode{Val: 0, Next: nil}
	var link = &dummy

	for _, v := range nodes {
		link.Next = &ListNode{Val: v}
		link = link.Next
	}

	return dummy.Next
}

func (lk *ListNode) IterLink() {
	fmt.Print("lk: ")
	for lk != nil {
		fmt.Print(lk.Val)
		lk = lk.Next
	}
}
