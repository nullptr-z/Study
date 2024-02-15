# 用于生成 gRPC 代码；.proto 文件支持 rpc 关键字

## 插件安装

```she
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest 新
```

## 编译

1. `protoc -I . hello.proto --go_out=.` 编译 RPC 代码；不再支持`plugins=grpc`
1. `protoc -I . hello.proto  --go-grpc_out=.` 编译 gRPC 代码

## go_package

option go_package = "package path/name"; 指定 go 的包名
option go_package = ".;name"; 直接在当前目录生成
