## derive_builder::Builder

#[builder(setter(into))] // 表示会去调用传入对象的`into`方法, 这个对象需要能 into 到 String
name: String

设置默认值\
#[builder(default="42")]
#[builder(default="vec![1, 2, 3]")]

#[builder(setter(into, strip_option))] // 将数据包装成一个 Option 类型

#[builder(setter(skip))] // 跳过，不进行设置

## derive_more::{Add, Display, From, Into}

需要给给结构体添加一些方法，比如 `Add`, `Display`, `From`, `Into` 等。

不用自己实现这些方法，而是使用 `derive_more` 宏来实现。

## Strum

各种 Enum 相关的宏 #[derive(EnumString, EnumCount, EnumIter, EnumDiscriminants, EnumIs, EnumMessage, VariantNames)]
