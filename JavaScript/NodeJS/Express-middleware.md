## 应用级别中间件

app.get() app.post() app.put() app.delete() app.patch()

```js
// next() 继续执行下一个中间件
app.use((req, res, next) => {});
app.use(path, (req, res, next) => {});
app.use([paths], (req, res, next) => {});
app.use(path, ...middleFn, (req, res, next) => {}); // 多个中间件
```

`app.all( path, (req, res, next) => {})` 为所有路由满足`path`的请求，添加中间件

> 路由模式匹配，: . ? \* ()，包: path-to-regexp

不带路由参数的中间件会依次向下执行，且总是执行

每个中间件都可以通过 `req, res`, 来`访问/修改`请求对象，响应对象

当调用 res.send()后,还可以调用 next()继续执行下一个中间件，但是不能再次调用 res.send()

## 自定义中间件

```js
function myMiddleware(obj) {
  return (req, res, next) => {
    console.log(obj);
  };
}

app.use(myMiddleware{ obj: 123 }));
```

> 满足条件跳过某些中间件

类似这种用法 `next('router')`

## 路由中间件

```js
const router = express.Router();

router.get("/hello", (req, res, next) => {
  res.send("hello world");
});
// 限定路由前缀
app.use("/api", router);
```

## 错误处理中间件

```js
app.use((req, res, next) => {
...
  try{

  } catch(err) {
    next(err) // 一定要传递错误对象err，否则执行下一个中间件
  }
}

...其他中间件

// 写在所有中间件之后
app.use((err, req, res, next) => {
  console.log(err);
  res.status(500).send("服务器错误");
});
```

## 404 中间件

```js
// 必须写在其他中间件之后
app.use((req, res, next) => {
  res.status(404).send("404 Not Found");
});
```
