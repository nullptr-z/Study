package main

func levelOrder(root *TreeNode) [][]int {
	var result [][]int
	var level []int
	stack := NewStack()
	stack.Push(root)
	count := stack.Len()

	for stack.Len() > 0 {
		node := stack.Pop()
		count--

		if node != nil {
			stack.Push(node.Left)
			stack.Push(node.Right)
			level = append(level, node.Val)
		}

		if count == 0 {
			count = stack.Len()
			if len(level) > 0 {
				result = append(result, level)
				level = []int{}
			}
		}
	}

	return result
}

type Queue[T TreeNode] struct {
	Inner []*T
}

func (s *Queue[T]) Pop() *T {
	len := len(s.Inner)
	if len < 1 {
		return nil
	}
	endVal := s.Inner[0]
	s.Inner = s.Inner[1:len]
	return endVal
}

func (s *Queue[T]) Push(item *T) {
	s.Inner = append(s.Inner, item)
}

func (s *Queue[T]) Len() int {
	return len(s.Inner)
}

func NewStack[T TreeNode]() Queue[T] {
	s := Queue[T]{Inner: []*T{}}
	return s
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}
