# [databse](https://pkg.go.dev/database/sql#DB)

## 常用查询方法

- Query
- QueryRow
- QueryContext
- QueryRowContext

## Rows

```go
type Rows struct{}
// 一些常用函数，伪代码

// 返回列的信息
ColumnTypes()([]*ColumType, error)
// 返回所有列名
Columns()([]*ColumType, error)
// 每次读一行，返回是否还有数据
Next() bool
// 每次读一个结果集，返回是否还有数据
NextResultSet() bool
// 把对结果放在对应字段
Scan(dest ...interface{}) error
// 关闭
Close() error
// 查询出错
Err()
```

## QueryRow
