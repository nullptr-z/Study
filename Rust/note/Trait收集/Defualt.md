## 关于一些技巧

如果定了结构，所有字段都型全部实现了 Default，那么可以直接使用 Default::default()来初始化结构体。一定程度上可以替代`new`函数。

```rs
#[derive(Default)]
struct A {
    a: Arc<i32>,
}

let a = A::default();

// 等价于

impl A {
    fn new() -> Self {
        Self {
            a: Arc::new(0),
        }
    }
}
```
