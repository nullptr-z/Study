## 应用场景
构建在http2之上, 所以叫做`gRPC over HTTP/2`

无法直接跟浏览器的很好的交互，通常需要`gRPC web`协助访问

对mobile、server to server场景可以很好的使用gRPC，因为他们都是基于http2的


## 构建一个gRPC服务
1. 创建`proto`文件, 用于定义`struct`, `enum`等数据结构。 和使用这些结构的`service`，这个service被称为`rpc service`
2. 使用`prost`编译`proto`文件，生成`rust`代码和添加`derive`。这里我们使用`tonic_build`来编译`proto`文件，因为他是基于`prost_build`的，对其扩展, 能生成更多的代码，比如`grpc service`定义的代码
3.  Client: gRPC提供完整的用于客户端调用的代码，客户端只需要链接并调用接口就可以了

在Client和Server发起调用`Call`之前，可以通过`interceptor`做额外的处理

## Tonic
基于`prost`，使用他来构建protobuf和gRPC的rust代码, 然后使用`tonic`来构建grpc服务

Prost并没有完整的生成`grpc service`代码，留给了grpc framework来实现，比如`tonic`

tonic通过实现[prost_build::ServiceGenerator](https://docs.rs/prost-build/0.7.0/prost_build/trait.ServiceGenerator.html) Trait 来生成protobuf数据结构的定义，和grpc service的定义

## 特性
* 支持`async/await`，通过`tokio`支持，作为底层的异步框架
* 支持`TLS`，通过`rustls`支持
* 继承了`twirp`体系的特性，可以使用`middleware`的支持

## 相关Create
[prost] (https://github.com/nullptr-z/rust-live/tree/main/create-practice/prost-live)

[hyper](http)

[twirp](http)
