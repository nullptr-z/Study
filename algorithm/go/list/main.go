package list

import "fmt"

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
