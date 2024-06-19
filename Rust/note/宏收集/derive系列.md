## derive_builder::Builder

**增强对结构体字段和方法的构建能力**

#[builder(setter(into))] // 表示会去调用传入对象的`into`方法, 这个对象需要能 into 到 String
name: String

设置默认值\
#[builder(default="42")]
#[builder(default="vec![1, 2, 3]")]

#[builder(setter(into, strip_option))] // 将数据包装成一个 Option 类型

#[builder(setter(skip))] // 跳过，不进行设置

## derive_more::{Add, Display, From, Into}

**提供了对标准库的实现**

`Add`, `Display`, `From`, `Into` 等 trait 的实现。

不用自己手动写这些方法，而是使用 `derive_more` 宏来实现。

## Strum

**主要用于 Enum 的各种宏, 以及和 String 的交互**

各种 Enum 相关的宏 #[derive(EnumString, EnumCount, EnumIter, EnumDiscriminants, EnumIs, EnumMessage, VariantNames, IntoStaticStr)]

#[strum(serialize_all = "snake_case", serialize = "newName", to_string = "newName")]
