https://serde.rs/

在实际软卡开发中，我们会用到 serde 来做数据转换，特别是跟网络和文件打交道的时候。

serde 库提供的了`#[derive(Serialize, Deserialize)]`宏

它支持提供了一个强大的中间层，我们在做数据转换的时候，只需要跟中间层交互，serde 会帮我们做了到其他格式的转换
