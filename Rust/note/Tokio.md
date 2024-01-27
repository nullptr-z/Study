## 核心机制

- 异步任务：Tokio 通过 Future 来表示异步任务，这些任务可以在后台并发执行。

- 执行器（Executor）：负责调度和执行异步任务。Tokio 提供了一个多线程的执行器，可以在多个线程上并发运行任务。

- Reactor（反应器）：用于监控和响应 I/O 事件（如网络或文件系统操作）。当一个异步任务需要等待 I/O 事件时，它会被挂起，而反应器会在该事件发生时重新唤醒任务。

- 任务调度：Tokio 使用基于工作窃取的策略来分配任务到不同的线程上，以实现负载均衡。

- 非阻塞 I/O：Tokio 提供了一系列的非阻塞 I/O 操作，使得任务在等待 I/O 时不会阻塞整个线程，从而提高效率。

- Waker 和 Context：这些机制用于在任务需要被挂起或重新唤醒时通知执行器。

非阻塞 I/O 和 Waker 好像配合起来实现了 Reactor