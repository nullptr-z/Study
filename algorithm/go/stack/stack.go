package stack

type Stack[T TreeNode] struct {
	Inner []*T
}

func (s *Stack[T]) Pop() *T {
	end := len(s.Inner) - 1
	endVal := s.Inner[end]
	s.Inner = s.Inner[0:end]
	return endVal
}

func (s *Stack[T]) Push(item *T) {
	s.Inner = append(s.Inner, item)
}

func (s *Stack[T]) Len() int {
	return len(s.Inner)
}

func NewStack[T TreeNode]() Stack[T] {
	s := Stack[T]{Inner: []*T{}}
	return s
}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}
