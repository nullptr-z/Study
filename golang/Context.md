<!-- version 1.7 -->

# Context

提供了多种方案，在不同场景下协调多个 goroutine 工作

- 官方提供的取代`<-chan struct{}`方案

```go
ctx, cancel := context.WithCancel(context.Background())
<-ctx.Done()
```
