## HTTP协议

### 请求
request 行:
请求方式     请求API（uri）   HTTP版本         结束符号
Method      Request-URI     HTTP-Version    CRLF
GET         /               HTTP/1.1        \r\n

uri: 是HTTP中的术语,URL去除 `IP:port`(host)的部分, 每个`/`一个节点
非管理员用户只能监听大于 1024 的端口

### 响应
HTTP版本    状态码    短语    CRLF

一个使用 HTTP 1.1 版本的响应例子，其状态码为 200，原因短语为 OK，没有
header，也没有 body：
HTTP/1.1 200 OK\r\n `(此处为header)` \r\n `(此处为body)`




