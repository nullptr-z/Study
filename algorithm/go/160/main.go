package main

import (
	"fmt"
)

func main() {
	res := getIntersectionNode(NewLinked([]int{4, 1, 8, 4, 5}), NewLinked([]int{5, 0, 1, 8, 4, 5}))
	res.IterLink()
}

func getIntersectionNode(headA, headB *ListNode) *ListNode {
	adsMap := make(map[string]bool)
	dummy := headA
	for dummy != nil {
		address := fmt.Sprintf("%p", dummy)
		adsMap[address] = true
		dummy = dummy.Next
	}

	dummy = headB
	for dummy != nil {
		address := fmt.Sprintf("%p", dummy)
		if adsMap[address] {
			return dummy
		}
		dummy = dummy.Next
	}

	return nil
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
