提供了接口文档、mocker 数据、集成测试一套方案

解决了后端 api 文档和前端 mocker 数据不一致问题

## 使用 docker 安装

https://github.com/fjc0k/docker-YApi

```sh
git clone https://github.com/fjc0k/docker-YApi.git
# 无法访问 github 的可使用国内镜像：
git clone https://gitee.com/fjc0k/docker-YApi.git
```

接下来，修改 docker-compose.yml 中 yapi-web 下的环境变量 YAPI_ADMIN_ACCOUNT 为你的管理员邮箱，YAPI_ADMIN_PASSWORD 为你的管理员密码。

最后，执行 docker-compose up -d 启动服务。

然后，通过 http://localhost:40001 即可访问 YApi。
