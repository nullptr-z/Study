## format 输出文档 https://doc.rust-lang.org/std/fmt/

`use std::fmt;`

```rs
    format!("format！");                // 与printf!类似，不过他是返回一个String，而不是打印
    println!("format！");               // 输出到终端：标准输出 io::stdout
    eprintln!("format！");              // 输出到终端：标准错误 io::stderr
    write!(f, "write!");                // 将格式化后的字符串写入缓冲区,也就是第一个参数f文件描述符
```
