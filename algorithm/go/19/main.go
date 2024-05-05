package main

import "fmt"

func main() {
	res := removeNthFromEnd(NewLinked([]int{1, 2, 3, 4, 5}), 2)
	res.IterLink()
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	var dummy = &ListNode{Val: 0, Next: head}
	var fast, slow *ListNode = dummy, dummy

	// 快的先走 N+1 步
	for i := 0; i < n+1; i++ {
		fast = fast.Next
	}
	// 两个在同时走
	for fast != nil {
		fast = fast.Next
		slow = slow.Next
	}
	slow.Next = slow.Next.Next

	return dummy.Next
}

func removeNthFromEnd2(head *ListNode, n int) *ListNode {
	var dummy = &ListNode{Val: 0, Next: head}
	var cur = head
	var size = 0
	for cur != nil {
		cur = cur.Next
		size++
	}
	cur = dummy
	for i := size - n; i > 0; i-- {
		cur = cur.Next
	}
	cur.Next = cur.Next.Next

	return dummy.Next
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
