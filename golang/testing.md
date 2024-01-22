## CLI

`go test packageName/modelName`

## 要求

import "testing"

file: 编译器自动识别`*_test.go`结尾的文件

func: 要求 `Test*` 开头的函数名

function argument: `t *testing.T`，它会提供测试相关的一些工具

## testing.T

t.ErrorF("")

## HttpTest

```go
httptest.NewRequest(http.MethodGet, uri, nil)
httptest.NewRecorder()
```
