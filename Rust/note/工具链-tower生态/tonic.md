
## 核心功能
基于`prost`，使用他来构建protobuf和gRPC的rust代码, 然后使用`tonic`来构建grpc服务

Prost并没有完整的生成`grpc service`代码，留给了grpc framework来实现，比如`tonic`

tonic通过实现[prost_build::ServiceGenerator](https://docs.rs/prost-build/0.7.0/prost_build/trait.ServiceGenerator.html) Trait 来生成 protobuf 数据结构的定义，和`grpc service`的定义

#### grpc service 如下定义
proto文件中的`service`定义
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

#### *Stream* 类型的`rpc`方法
`Stream`类型的结构可以`poll_next()`不断地获取下一个{ `message`, `status` }，直到`stream`结束


## 代码特征
他会对 proto 文件中的 message 结构添加`::prost::Message` Trait，这样就可以使用`prost`继承来的的方法来序列化(Serialize)和反序列化(Deserialize, UnDeserialize)这些结构


## Builder Attributes
类似 `tonic_build`，他也有一些`Builder Attributes Struct`，可以在`proto`文件中添加，来控制`tonic`的行为, 或者添加属性



## 核心数据结构
Struck: `Request`, `Response`, `Status`, `Streaming`

Trait: `IntoRequest`, `IntoResponse`, `IntoStreamingRequest`, `IntoStreamingResponse`



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
* `new`方法来创建一个`Request`结构
* 和一系列的`with_xxx`方法来获取和处理`Request`结构的属性
* 和一系列的`metadata*`方法来获取和处理`metadata`属性
* 和一系列的`extensions*`方法来获取和处理`extensions`属性
* 等。。。

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

它封装了`futures::Stream` Trait，所以可以使用`futures::Stream` Trait的方法来处理`Streaming`结构
