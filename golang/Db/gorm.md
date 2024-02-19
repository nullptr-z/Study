## 建表

db.AutoMigrate(&User{})

[gorm](https://gorm.io/docs) 网站有中文语言支持

术语 migration，指将代码中的结构体映射成数据库中的表

## 设计表字段

```go
`gorm:"column:name"; type:"varchar(25)";index:idx_name;unique;default:"value";references:Code; primarykey`
```

references 的作用是指定一个外键；默认使用的对方主键

https://gorm.io/zh_CN/docs/models.html

## 输出 API 背后的 sql 语句

通过`gorm`库提供 logger 输出

[logger](https://gorm.io/zh_CN/docs/logger.html)

## 查询语句

- First：根据主键查询，正序
- Last：根据主键查询，倒叙
- Tack: 非排序直接第一条
- Find: 查询全部
- Where：条件语句

## Join 查询

.Join("外键表结构名字") 默认使用主键匹配，也可以直接写`join on`sql 语句

## 软删除

通过设置`gorm.DeleteAt`来设置软删除，设置后 Delete api 只会在设删除设置删除时间，而不是真的删除，之后的查询也不会查询到这条数据

需要`Unscoped()` 来查询软删除的数据；`Unscoped().Delete`会永久删除

## 注意事项

- updates: 会忽略没有填写的字段和，赋予默认值的字段，例如字符串"",数值 0，因为 ORM 不敢随意猜测我们到底是要跟新这个值还是忽略；
  需要 `sql.NullString`,`sql.NullInt` 等类型来强制设置零值; update_at 会改变

## 约定

- 数据库表字节默认映射成`snake case`，`grom:"column:name_suffix"`来改变这个行为
