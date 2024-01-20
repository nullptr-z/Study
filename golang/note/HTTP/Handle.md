## HandleFunc

HandlerFunc 是一个是适配器, 可以将某个具有适当函数签名的函数，转换成为一个 Handler

实现：

```go
type HandlerFunc func(ResponseWriter, *Request)

// ServeHTTP calls f(w, r).
func (f HandlerFunc) ServeHTTP(w ResponseWriter, r *Request) {
	f(w, r)
}
```

使用：

```go
func welcome(w http.ResponseWriter, r *http.Request) {
	w.Write([]byte("welcome"))
}

http.HandleFunc("/index", welcome)
// 等价方式，使用HandlerFunc适配器
http.Handle("/index", http.HandlerFunc(welcome))
```

## http.Handle

`http.Handle(router, handle)`,接受一个路由字符串，和一个 handle

## 自定义 Handle

可以看做 `HandleFunc` 实现原理或拆解

实现了 http.ServeHTTP 就可以被用作 Handle

```go
type myHandle struct{}

func (slf *myHandle) ServeHTTP(rsp http.ResponseWriter, req *http.Request) {
rsp.Write([]byte("<div>hello world</div>"))
}

fn main(){
  mh := myHandle{}
  server := http.Server{Addr: addr, Handler: &mh}
  server.ListenAndServe()
}
```

用作路由

```go
type helloHandle struct{}

func (slf *helloHandle) ServeHTTP(rsp http.ResponseWriter, req *http.Request) {
	rsp.Write([]byte("<div>hello world</div>"))
}

mh := helloHandle{}
http.Handle("/hello", &mh)
```
