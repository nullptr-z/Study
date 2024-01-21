## Open

open 函数并不会直接连接数据库，只是设置了连接到数据库的参数

## sql.DB

两种主要的使用方式

- 放在全局，到处都能用
- 通过函数之间传递使用

## 获取驱动 db.Drive

不同数据库有不同的驱动包, 他们需要实现 driver.Driver 接口，然后就可以通过 sql.Register 来注册了，这屏蔽了不同数据库之间的差异

`sql.Register("sqlserver", &drv{})`

注册工作通常是驱动包内部完成了，使用时只需要引入包即可

## Context

验证连接是否有效

```go
// 创建连接
ctx := db.Background()
// 验证连接
context := db.PingContext(ctx)
```

## 使用 PostgreSql

```sh
# 下载驱动包
go get -u github.com/lib/pq
```

引入：

```go
import _ "github.com/lib/pq"

const (
  host     = "localhost"
  port     = 5432
  user     = "your_username"
  password = "your_password"
  dbname   = "your_database_name"
)
```
