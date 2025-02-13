## Clone & ToOwned

_将借用数据（如 &str）转换为拥有所有权的数据（如 String）_

clone 和 to_owned 在很多情况下都可以用来创建一个数据的拷贝。它们的区别主要在于它们对应的 trait 和实现方式。

clone 对应的是 Clone trait，几乎所有的类型都可以实现 Clone trait，来提供一个复制自身的方法。当你调用 clone 方法时，你会得到一个和原始数据一样的副本。

to_owned 对应的是 ToOwned trait。ToOwned trait 的主要目的是从借用的版本创建一个拥有的版本。例如，从&str 创建 String，或者从&[T]创建 Vec<T>。ToOwned 通常用于更抽象的上下文，例如在你不知道具体类型，只知道它是借用的情况下。

在实际使用中，如果你的数据类型实现了 ToOwned，那么 clone 和 to_owned 在功能上是一样的。但是有些情况下，例如处理&str 和 String，使用 to_owned 更加明确，因为它明确表示你是在从一个借用的版本创建一个拥有的版本。

总的来说，clone 和 to_owned 在大多数情况下可以互换

**举例**：
专为 Borrow<T> 设计（如 &str → String, &[T] → Vec<T>）
本质上是克隆，但语义强调 “从借用类型创建一个有所有权的类型”

**实现细节：**
许多类型（如 String）的 to_owned() 直接调用 clone()
