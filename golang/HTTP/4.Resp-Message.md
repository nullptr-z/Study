## ResponseWriter,`*w`

它是一个接口; http 包内部有个`type response struct`私有的，实现了 ResponseWriter

`response struct`是一个胖指针结构，所以传递是很高效的

## w.Write()

如果没有设置`content type`，则发送数据的前 512 个字节会被用来推测出 content type

```go
// 自动将数据转换为字节数组再发送
fmt.Fprintln(w, msg)
// 更为底层一点点的方法，需要手动处理数据
w.Write([]byte(msg))
```

## w.Header()

用于在响应前，获取和修改响应头; 调用`w.WriteHeader`后的所有修改都不会生效

```go
// 返回json格式的数据
func respJson(w http.ResponseWriter, r *http.Request) {
  w.Header().Add("Content-Type", "application/json")
  resMsg := RespMessage{"zheng", []string{"China", "Shanghai"}}
  json, _ := json.Marshal(resMsg)
  fmt.Fprintln(w, string(json))
}
```

## WriteHeader

设置一个状态码

如果没有显示调用，内部默认行为是`WriteHeader(http.StatusOK)`

## 一些内置的 Response

- NodeFound 响应 404
- ServeFile 响应给定路径的文件
- ServeContent 任何实现了 io.ReadSeeker 结构的任何结构，都可以它发送给接收方
- Redirect 让客户端重定向到指定 RUL
