# RPC

RPC 是一种编程模型，允许一个程序调用另一个程序或者远程服务的子程序(函数)，就像调用本地函数一样，而不必关心底层通信细节。RPC 可以基于不同的传输协议实现，包括 HTTP、TCP、UDP 等，来实现远程过程调用。

通过一些协议头，标记服务端提供的服务信息，例如远程方法名`Method: hello.sayhi`

# 什么是 gRPC

构建在 http2 之上, 所以叫做`gRPC over HTTP/2`，双全工；

无法直接跟浏览器的很好的交互，通常需要`gRPC web`协助访问

对 mobile、server to server 场景可以很好的使用 gRPC，因为他们都是基于 http2 的

- 数据协议: protobuf
- 传输协议: http2

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

- 4 字节消息长度
- 1 字节是否压缩
- 最后拼接上消息内容

## 状态码

[StatusCode](https://github.com/grpc/grpc/blob/master/doc/statuscodes.md)

## MetaData

类似 http 的请求头，`K:[V]` 键值对支持，V 是字符串数组
