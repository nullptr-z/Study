Pipeline 主要是一种网络优化。它本质上意味着客户端缓冲一堆命令并一次性将它们发送到服务器。这些命令不能保证在事务中执行。这样做的好处是节省了每个命令的网络往返时间(RTT)

多个 Redis 命令一次性发送过去，减少 RTT

```
pipe = redis.Pipeline()
..commands..
pipe.exec()
```
