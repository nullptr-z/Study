// 简单工厂，不属于 23 中设计模式，因为不符合开闭原则
// 特性：通过一个类来创建其他类的实例，被创建的实力通常有用同一个父类
// 使用者: 业务只需要和工厂和抽象打交道，不需要知道具体实现
// 有点：对象的创建和使用分离，只需要记住实现的名称(工厂的参数)
// 缺点：增加了工厂负担，每次增加一个实现都需要在工厂添加一个实例，不符合开闭原则

package main

import "fmt"

// =======抽象层========
type Fruit interface {
	Show()
}

// =======实现========
type Apple struct {
	Fruit // 表示继承关系，实际 go 不需要
}

type Banana struct {
	Fruit // 表示继承关系，实际 go 不需要
}

type Orange struct {
	Fruit // 表示继承关系，实际 go 不需要
}

func (slf *Apple) Show() {
	fmt.Println("the apple")
}
func (slf *Banana) Show() {
	fmt.Println("the banana")
}
func (slf *Orange) Show() {
	fmt.Println("the Orange")

}

// =======工厂========
func Factory(name string) Fruit {
	switch name {
	case "banana":
		return new(Banana)
	case "apple":
		return new(Apple)
	case "orange":
		return new(Orange)
	}
	return nil
}

func main() {
	var f Fruit

	f = Factory("banana")
	f.Show()

	f = Factory("apple")
	f.Show()

	f = Factory("orange")
	f.Show()
}
