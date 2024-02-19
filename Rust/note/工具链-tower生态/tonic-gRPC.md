## 核心功能

基于`prost`，使用他来构建 protobuf 和 gRPC 的 rust 代码, 然后使用`tonic`来构建 grpc 服务

Prost 并没有完整的生成`grpc service`代码，留给了 grpc framework 来实现，比如`tonic`

tonic 通过实现[prost_build::ServiceGenerator](https://docs.rs/prost-build/0.7.0/prost_build/trait.ServiceGenerator.html) Trait 来生成 protobuf 数据结构的定义，和`grpc service`的定义

#### grpc service 如下定义

proto 文件中的`service`定义

```proto
service Test {
    rpc Hello (HelloRequest) returns (HelloResponse);
}
```

编译后他会生成如下代码:

```rust
//
pub mod Test_server{ ... }
//
pub mod Test_client{ ... }
```

## 构建一个 gRPC 服务

1. 创建`proto`文件, 用于定义`struct`, `enum`等数据结构。 和使用这些结构的`service`，这个 service 被称为`rpc service`
2. 使用`prost`编译`proto`文件，生成`rust`代码和添加`derive`。这里我们使用`tonic_build`来编译`proto`文件，因为他是基于`prost_build`的，对其扩展, 能生成更多的代码，比如`grpc service`定义的代码
3. Client: gRPC 提供完整的用于客户端调用的代码，客户端只需要链接并调用接口就可以了

在 Client 和 Server 发起调用`Call`之前，可以通过`interceptor`做额外的处理

#### _Stream_ 类型的`rpc`方法

`Stream`类型的结构可以`poll_next()`不断地获取下一个{ `message`, `status` }，直到`stream`结束

## 代码特征

他会对 proto 文件中的 message 结构添加`::prost::Message` Trait，这样就可以使用`prost`继承来的的方法来序列化(Serialize)和反序列化(Deserialize, UnDeserialize)这些结构

## Builder Attributes

类似 `prost_build`，他也有一些`Builder Attributes Struct`，可以在`proto`文件中添加，来控制`tonic`的行为, 或者添加属性

## 核心数据结构

Struck: `Request`, `Response`, `Status`, `Streaming`

Trait: `IntoRequest`, `IntoResponse`, `IntoStreamingRequest`, `IntoStreamingResponse`

## 辅助功能

`#[tonic::async_trait]`
tonic 对`async_trait`的支持，不需要在项目中额外添加`async_trait`的依赖

#### Request

```rust
/// A gRPC request. T 就是 Message 结构
pub struct Request<T> {
    /// 包含了Message结构的信息
    pub message: T,
    /// 包含了这个请求http header的信息
    pub metadata: MetadataMap,
    /// 包含了这个请求携带的额外信息，可以用来存储一些额外的信息
    pub extensions: Extensions,
}
```

一些 Request function:

- `new`方法来创建一个`Request`结构
- 和一系列的`with_xxx`方法来获取和处理`Request`结构的属性
- 和一系列的`metadata*`方法来获取和处理`metadata`属性
- 和一系列的`extensions*`方法来获取和处理`extensions`属性
- 等。。。

#### Response

他的`struct`,`function` 和`Request`类似

#### Status

```rust
/// A gRPC status.
pub struct Status {
    /// 对应的http状态码的枚举，grpc的状态码是基于http状态码的
    pub code: Code,
    /// The status message.
    pub message: String,
    /// The status details.
    pub details: Vec<u8>,
}
```

#### Streaming

主要功能是`poll_next()`来获取下一个{ `message`, `status` }，直到`stream`结束

它封装了`futures::Stream` Trait，所以可以使用`futures::Stream` Trait 的方法来处理`Streaming`结构

## 相关 Create

[prost] (https://github.com/nullptr-z/rust-live/tree/main/create-practice/prost-live)

[hyper](http)

[twirp](http)
