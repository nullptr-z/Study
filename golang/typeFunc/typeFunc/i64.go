package typeFunc

import (
	"fmt"
)

type I64 int64

func (i I64) String() string {
	return fmt.Sprintf("ints:%d", i)
}
