# 为什么需要 gRPC

## 应用场景

构建在 http2 之上, 所以叫做`gRPC over HTTP/2`，双全工

无法直接跟浏览器的很好的交互，通常需要`gRPC web`协助访问

对 mobile、server to server 场景可以很好的使用 gRPC，因为他们都是基于 http2 的

## 构建一个 gRPC 服务

1. 创建`proto`文件, 用于定义`struct`, `enum`等数据结构。 和使用这些结构的`service`，这个 service 被称为`rpc service`
2. 使用`prost`编译`proto`文件，生成`rust`代码和添加`derive`。这里我们使用`tonic_build`来编译`proto`文件，因为他是基于`prost_build`的，对其扩展, 能生成更多的代码，比如`grpc service`定义的代码
3. Client: gRPC 提供完整的用于客户端调用的代码，客户端只需要链接并调用接口就可以了

在 Client 和 Server 发起调用`Call`之前，可以通过`interceptor`做额外的处理

## 特性

- 支持`async/await`，通过`tokio`支持，作为底层的异步框架
- 支持`TLS`，通过`rustls`支持
- 继承了`twirp`体系的特性，可以使用`middleware`的支持

## Request-header

一些常用的 Header

- Method: 都是 POST
- status 与 http
- grpc-status 1..n grcp 自己规定的状态码
- Scheme: http/https
- Content-type: application/grpc +[proto, json, ...等序列化工具]
- grpc-encoding: gzip ...
- Message-EnCoding: grpc-encoding
- Path: 请求路径，服务名+结构名/服务名(具体的某个函数名字)

## Length-Prefixed-Message

- 1 字节是否压缩
- 4 字节消息长度
- 最后拼接上消息内容

## 相关 Create

[prost] (https://github.com/nullptr-z/rust-live/tree/main/create-practice/prost-live)

[hyper](http)

[twirp](http)
