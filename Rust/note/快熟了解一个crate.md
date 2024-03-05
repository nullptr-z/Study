_配合流程图工具，边画边读源码，对理清脉络更佳_
_深入阅读时候可以使用 gdb_
_分享，讲给别人_

# Trait

## required Methods

必要实现的方法

## Povided Methods

缺省方法，自动实现方法；当然也可以自己重载他们

## foreign types

这个 crate 实现了哪些 Trait

## implementors

为哪些 crete 实现了当前这个 Trait

# Struct

## Methods

扫一下定义了哪些方法，快速看签名和注释

## Trait implementations

扫一下实现了哪些 Trait；

平时也应该多熟悉常见的 Trait，这对也读源码能力提升很有帮助；
经常在遇到一个问题的时候，不知道如何将一个类型转化到目标类型，但如果知道他们共同的 Trait，就可以通过这个 Trait 转换过去

# 了解作者是如何使这个库用的

查看项目 examples 或 tests 目录下的测试用力

再自己写一些示例代码

# 深入具体实现

从感兴趣的部分开始，围绕这一个特定情景展开阅读

这通常效率不是很高的，是为了明白原理的目的，非必要情况是不必要的，大多视情况需要如何使用就行了

# 我们自己的数据结构

也应该尽可能实现需要的标准 trait，包括但不限于：AsRef、Borrow、Clone、Debug、Default、Deref、From<T>、Hash、 Send / Sync、IntoIterator、PartialEq/Eq、、PartialOrd/Ord （如果是个集合类型）等。
