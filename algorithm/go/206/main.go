package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

func reverseList(head *ListNode) *ListNode {
	var res *ListNode
	for head != nil {
		next := head.Next
		head.Next = res
		res = head
		head = next
	}

	return res
}

func reverseList2(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	next := reverseList2(head.Next)
	head.Next.Next = head
	head.Next = nil
	return next
}
func reverseLists(head *ListNode) *ListNode {
	dummy := ListNode{Next: nil}
	cur := &dummy

	stack := []ListNode{}
	for head != nil {
		stack = append(stack, ListNode{Val: head.Val})
		head = head.Next
	}

	size := len(stack)
	for i := 0; i < size; i++ {
		fmt.Println("i:", i)
		top := len(stack) - 1
		cur.Next = &stack[top]
		cur = cur.Next
		stack = stack[:top]
	}

	return dummy.Next
}
