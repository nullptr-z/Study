## 建议配置

```sh
go env -w GO111MODULE=on # 开启module模式
go env -w GOPROXY=https://goproxy.cn    # 换包源
```

## 环境变量

```sh
export GOPATH=~/go
export PATH=$PATH:$GOPATH/bin
```

## mod 常用命令

```sh
go mod init	    |   生成 go.mod 文件
go mod download	|   下载 go.mod 文件中指明的所有依赖
go mod tidy	    |   整理现有的依赖
go mod graph	  |   查看现有的依赖结构
go mod edit	    |   编辑 go.mod 文件
go mod vendor	  |   导出项目所有的依赖到 vendor 目录
go mod verify	  |   校验一个模块是否被篡改过
go mod why	    |   查看为什么需要依赖某模块
go mod tidy     |   重载依赖
```

替换包版本 `go mod edit -replace = source replace_source`

## Env

```sh
go env                |   查看go的环境变量
go env -w name=value  |   value写入name环境变量
```

env 字段说明：

```sh
GOSUMDB     校验包的原地址，如果设置了GOSUMDB，就失效了
```
