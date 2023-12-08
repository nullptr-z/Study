## HttpServer 类

HttpServer 类型负责为 HTTP 请求提供服务。

HttpServer 接受应用程序工厂作为参数，并且应用程序工厂必须具有 Send + Sync 约束。在“多线程”一节有更多信息。

要绑定到特定的套接字（socket）地址，必须使用 `bind()` 方法，并且可以多次调用它。
要绑定 ssl 套接字（socket），应使用 `bind_openssl()` 方法或者 `bind_rustls()` 方法。
要运行 HTTP 服务器，请使用 `HttpServer::run()` 方法。

run() 方法返回 Server 类型的实例，Server 类型的方法可用于管理 HTTP 服务器：

- pause() - 暂停接受传入的连接
- resume() - 重新开始接受传入的连接
- stop() - 停止处理传入的连接，停止所有工作线程，并退出
- shutdown_timeout(s) - 设置 s 秒关闭链接,默认关闭超时设置为 30 秒

## 缺省头

附加的,非标准协议需要的字段,自定义的部分

## SSE

长连接、单向通信、运行服务器主动推送、文本数据传输

Server-Sent Events（SSE）用于在客户端和服务器之间建立单向通信通。它允许服务器向客户端推送事件，而不需要客户端发起请求。SSE 的设计目的是实现轻量级、简单且可靠的服务器到客户端的实时通信。
