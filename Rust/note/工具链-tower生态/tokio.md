## \#[tokio::main]
将 `async fn main`转换为普通 `fn main`函数
```rust
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // 这里为 async fn main中的代码
        println!("hello");
    })
}
```
