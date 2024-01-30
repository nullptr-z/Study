两个函数等效

```rs
// 语法糖
async fn test_fn() -> () {
  fn().await
}

fn test_fn1() -> Future<Output = ()> {
  async{
    fn()
  }
}
```
