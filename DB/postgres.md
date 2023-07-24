## 安装

```shell
apt install pgcli

# 进入数据库控制台
pgcli -u postgres_name -w postgres_password -p 5433

```

## 建库/建表

````sql
-- 建数据库
create database test;
-- 创建 schema
-- create schema test;
-- 创建表
create table test (
    id int primary key,
    name varchar(20)
);
-- 插入数据
insert into test values (1, 'test');
-- 查询数据
select * from test;
-- 删除表
drop table test;
-- 删除数据库
drop database test;
-- 退出数据库
\q

## 切换数据 database
```sql
\c database_name
````

```

## 表字段常见类型

> 数值类型

- serial,bigserial 自增主键
- smalint 2 字节 有符号整型；
- intger 4 字节 有符号整型；
- decimal(p,s) 可变精度浮点数，最大为 p，小数点后最多 s 位
- double 8 字节 双精度浮点数

> 字符类型

- char(n) n 字节 预设定长度的字符串，不足 n 位会用空格补齐
- varchar(size) 可变长度的字符串

> 日期类型

- timestamp 8 字节 日期和时间
- date 4 字节 日期
- time 4 字节 时间

> 其他类型

- bool 布尔类型
```

## 备份/恢复库

```shell
pg_dump source_db -> source_db.bak # 备份数据库

pg_dumpall > source_db.bak # 备份所有数据库

psql -f source_db.bak < database # 恢复数据库
```

## 常用命令

一下命令运行在 postgres cli

```shell
\c database_name # 切换数据库
\l # 查看数据库里列表
\d # 查看当前数据库所有表
\d table_name # 查看表结构
\du # 查看用户列表
\h select # 查看 select 命令帮助
\q # 退出
\? # 查看所有命令帮助,包括以上命令的说明
```
