## 常用装饰器

```ts
@Request(), @Req()	req
@Response(), @Res()*	res
@Next()	next
@Session()	req.session
@Param(key?: string)	req.params / req.params[key]
@Body(key?: string)	req.body / req.body[key]
@Query(key?: string)	req.query / req.query[key]
@Headers(name?: string)	req.headers / req.headers[name]
@Ip()	req.ip
@HostParam()	req.hosts
```

## session

```sh
yarn add express-session --save

yarn add @types/express-session --D
```

name: 设置名字
secret: 使用 session secret 对数据进行签名，以防止数据被篡改
rolling: 每次请求都会刷新 cookie
cookie: 设置 cookie,{ maxAge:"过期时间，负数是关闭浏览器页面就过期"}
