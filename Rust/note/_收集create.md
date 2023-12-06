features = ["full" ] // full features 意为,加载包的全部功能

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
