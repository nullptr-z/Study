// 单例模式,懒汉式式，首次初始化时创建对象实例，之后直接返回实例
// 保证一个类只有一个实例对象，可供其他模块使用，只读，不可被更改
// 不同的语言可能要借用不同方法来封装，const private ..

package main

import (
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

type singe struct{}

func (*singe) Show() {
	fmt.Println("I'm Singe")
}

var lock sync.Mutex  // 初始化时加锁，防止多线程重复初始化
var initialize int32 // 优化，防止过多上锁

var ptr *singe

func GetSinge() *singe {
	// 是否已经初始化
	if atomic.LoadInt32(&initialize) != 0 {
		fmt.Println("已经初始化了")
		return ptr
	}
	// 加锁
	lock.Lock()
	defer lock.Unlock()
	// 初始化
	if ptr == nil {
		fmt.Println("还没初始化")
		atomic.StoreInt32(&initialize, 1)
		ptr = new(singe)
	}

	return ptr
}

// 方式而使用 go 的 once函数，仅执行一次, 线程安全的
var onec sync.Once

func GetSinge_once() *singe {
	onec.Do(func() {
		ptr = new(singe)
	})

	return ptr
}

func main() {
	go func() {
		s := GetSinge()
		s.Show()
	}()

	go func() {
		s := GetSinge()
		s.Show()
	}()

	time.Sleep(time.Second)
}
