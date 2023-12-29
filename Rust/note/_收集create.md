## Serde

[serde(rename_all = "snake_case")] 蛇形命名所有字段（域）

[serde(skip_serializing_if = "Option::is_none")] 满足条件不进行序列化

[serde(rename_all = "snake_case")] Struct、Enum 中的全部字段`snake_case`

[serde(rename = "text-embedding-ada-002")] 字段序列化名称

[serde(default)] 表示在反序列化时，如果输入数据中缺少某个字段，就使用该字段的默认值。

[serde(untagged)] 表示 Serde 库将尝试匹配 JSON 数据和 Rust 结构体或枚举，而不考虑 JSON 对象中的字段名称(key)。这对于处理不同形式的 JSON 数据非常有用

## shellexpand

是一个用于展开和解析 shell 风格环境变量和波浪号扩展的库，~波浪符号被解析成系统用户根目录

## tower

网络生态圈

## rust-embed

将制定目录和文件资源打包到二进制的可执行文件中

## lazy_static

是 Rust 编程语言中一个非常有用的宏库，它提供了一种延迟计算静态变量的方法。中文翻译为“惰性静态变量”。第一次运行时确认它的值

在 Rust 中，静态变量是一种只分配一次内存的变量，通常在编译时就确定了它的值。但是，有时候我们需要在程序运行时才能确定静态变量的值，这时就可以使用 lazy_static 宏库。

由于 static const 都需要在编译时值是已知的，无法用于需要执行才可得知具体值的场景，例如,无法赋予 Result,Option 的值

## strum

枚举转字符串，默认转换成枚举字段名一样

strum = "0.25.0"
strum_macros = "0.25.3"

转换成 snake 格式：`#[strum(serialize_all = "snake_case")]`
转换成指定字符串：`#[strum(serialize = "whisper-1")]`

## 构建者模式-Builder

`derive_builder = "0.12.0"`

在 new 一个参数很多的 structure 时需要填写所有参数很不方便，这个 create 可以让我们只需要提供向提供的，帮我们生成其他缺省值

##### Maroc

#[builder(pattern = "mutable")] 表示生成的 Builder 是可变的

> default:

```rs
#[builder(default = "String::new()")]
#[builder(default = "0")]
#[builder(default)]
```

## 持久化存储 DB

RocksDB: 是 Facebook 在 Google 的 levelDB 基础上开发的嵌入式 KV store，用 C++ 编写的; 更加合适，因为它在各种复杂的生产环境中经历了千锤百炼

sled: 是 Rust 社区里涌现的优秀的 KV store，对标 RocksDB; 使用起来更简单
