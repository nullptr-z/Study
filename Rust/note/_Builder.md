构建者模式

`derive_builder = "0.12.0"`

在 new 一个参数很多的 structure 时需要填写所有参数很不方便，这个 create 可以让我们只需要提供向提供的，帮我们生成其他缺省值

## Maroc

#[builder(pattern = "mutable")] 表示生成的 Builder 是可变的

> default:

```rs
#[builder(default = "String::new()")]
#[builder(default = "0")]
#[builder(default)]
```
