// 单例模式,饿汉式，软件启动时就创建
// 保证一个类只有一个实例对象，可供其他模块使用，只读，不可被更改
// 不同的语言可能要借用不同方法来封装，const private ..

package main

import "fmt"

type singe struct{}

func (*singe) Show() {
	fmt.Println("I'm Singe")
}

var ptr *singe = new(singe)

func GetSinge() *singe {
	return ptr
}

func main() {
	s := GetSinge()
	s.Show()
}
