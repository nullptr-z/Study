多版本并发控制 Multiply Version Concurrent Control

- 当前读，读取的最新数据
- 快照读，读取的历史版本

- 版本控制：通过为数据的每次修改创建新的版本，同时保留旧版本，来支持并发访问。
- Read View：确定事务可以看到哪些数据版本，保证了读取操作的一致性。
- Undo Log：记录数据的旧版本，支持数据的回滚操作，并为 MVCC 提供必要的数据版本。

## 隔离级别-可见性

RR: 可重复读，默认
RC：读取已提交
读取未提交
串行化

## 1.隐藏列

- DB_RTX_ID: 最后一次创建、修改的事务 ID
- DB_ROW_ID：数据行号，也叫隐藏主键
- DB_ROLL_PTR：回滚指针

## 2.回滚日志

当不同事务对一条数据记录做修改时，会形成一条线性的日志链，链首就是最新的记录，链尾是最早的记录

## 3.读视图 Read View

事务在进行`读快照`时产生的视图信息

select 在只在第一次触发`当前读`,之后都是`快照读`
update delete insert 语句触发`当前读`

- trx_list: 系统活跃的事务 ID
- up_list_id: 列表中最小的 事务 ID
- low_limit_id: 系统尚未分配的下一个事务 ID

[参考](https://www.bilibili.com/video/BV1EM4y1c7nk?p=11&vd_source=29954a52608497ee1a304ca105f8cb17)
