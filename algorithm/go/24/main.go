package main

import (
	"fmt"
)

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

func main() {
	res := swapPairs(NewLinked([]int{1, 2, 3, 4}))
	fmt.Println("res:", res)
}

func swapPairs(head *ListNode) *ListNode {
	var dummyHead = &ListNode{Val: 0, Next: head}
	var dummy = dummyHead

	for dummy.Next != nil && dummy.Next.Next != nil {
		node1 := dummy.Next
		node2 := dummy.Next.Next

		// 指向节点 3
		node1.Next = node2.Next
		dummy.Next = node2
		dummy.Next.Next = node1

		dummy = node1
	}

	return dummyHead.Next
}
