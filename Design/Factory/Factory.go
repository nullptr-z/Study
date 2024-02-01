// 工厂方法模式
// 面向抽象层开发
// FruitAbs 抽象对象-水果类
// FruitFactory 抽象工厂函数-生产水果的工厂
// 添加新水果时，只需要实现这两个抽象就行，完全完全解耦

package main

import "fmt"

// 工厂模式

// =======水果类-抽象方法========
type FruitAbs interface {
	Name()
}

// =======工厂-抽象方法========
type FruitFactory interface {
	CreteFruit() FruitAbs
}

// =======实现========
type Apple struct {
	FruitAbs // 表示继承关系，实际 go 不需要
}

type Banana struct {
	FruitAbs // 表示继承关系，实际 go 不需要
}

type Orange struct {
	FruitAbs // 表示继承关系，实际 go 不需要
}

func (slf *Apple) CreteFruit() FruitAbs {
	var f FruitAbs = new(Apple)
	return f
}
func (slf *Banana) CreteFruit() FruitAbs {
	var f FruitAbs = new(Banana)
	return f
}
func (slf *Orange) CreteFruit() FruitAbs {
	var f FruitAbs = new(Orange)
	return f
}

func (slf *Apple) Name() {
	fmt.Println("the apple")
}
func (slf *Banana) Name() {
	fmt.Println("the banana")
}
func (slf *Orange) Name() {
	fmt.Println("the Orange")

}

func main() {
	var fruit FruitAbs
	var fact FruitFactory

	fact = new(Apple)
	fruit = fact.CreteFruit()
	fruit.Name()

	fact = new(Banana)
	fruit = fact.CreteFruit()
	fruit.Name()

	fact = new(Orange)
	fruit = fact.CreteFruit()
	fruit.Name()
}
