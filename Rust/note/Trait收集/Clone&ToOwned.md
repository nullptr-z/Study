## Clone & ToOwned

clone和to_owned在很多情况下都可以用来创建一个数据的拷贝。它们的区别主要在于它们对应的trait和实现方式。

clone对应的是Clone trait，几乎所有的类型都可以实现Clone trait，来提供一个复制自身的方法。当你调用clone方法时，你会得到一个和原始数据一样的副本。

to_owned对应的是ToOwned trait。ToOwned trait的主要目的是从借用的版本创建一个拥有的版本。例如，从&str创建String，或者从&[T]创建Vec<T>。ToOwned通常用于更抽象的上下文，例如在你不知道具体类型，只知道它是借用的情况下。

在实际使用中，如果你的数据类型实现了ToOwned，那么clone和to_owned在功能上是一样的。但是有些情况下，例如处理&str和String，使用to_owned更加明确，因为它明确表示你是在从一个借用的版本创建一个拥有的版本。

总的来说，clone和to_owned在大多数情况下可以互换
