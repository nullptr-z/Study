用于微服务的注册中心管理

## docker 安装

```sh
docker run -d -p 8500:8500 -p 8300:8300 -p 8301:8301 -p 8302:8302 -p 8600:8600/udp consul:1.15.4 consul agent -dev -client=0.0.0.0
# 自动重启
docker container update --restart=always 容器ID/名字
```

管理界面 http://localhost:8500/ui/dc1/services

## 解析 DNS

`localhost:8600` 服务, 用于解析微服务的地址

`dig` 命令使用指定的 DNS 服务器来解析

```sh
dig @localhost -p 8600 consul.service.consul SRV
```

## 配置

[注册服务](https://developer.hashicorp.com/consul/api-docs/agent/service#register-service)
