## 常用

```
g.use(middle,middle2) 注册全局中间件
g.get('/', middle, handel) 路由中间件
g.Group('/', middle) 路由组中间件

g.Next() 立即执行下一个中间件，默认要当前中间件执行完成才会执行；迭代 Handler 切片
g.About() 放弃执行后续中间件
```

## 中间件通信

中间件：m1,m2

```go
m1.Set("name","zhou")
...
m2.Get("name") // MustGet、GetString
```
