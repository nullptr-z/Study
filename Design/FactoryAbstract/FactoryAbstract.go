// 抽象工厂模式
// 用于面向产品族的进行抽象，在产品族成员不变的场景下使用
// 例：确定了有三种水果，【苹果、香蕉、桃子】，这就是一个产品族；如果后期还需要添加一个【梨】，那不适合使用抽象工厂，用工厂模式即可
// 添加产品线的时候是横向添加的，符合开闭原则；但是不能改变产品族的抽象

package main

import "fmt"

// ==========抽象产品族=========
type AppleAbs interface {
	EatApple()
}
type BananaAbs interface {
	EatBanana()
}
type PeachAbs interface {
	EatPeach()
}

// ==========抽象工厂=========
type FruitFactory interface {
	CreateApple() AppleAbs
	CreateBanana() BananaAbs
	CreatePeach() PeachAbs
}

// ==========实现=========
// 中国的产品生产线
type ChinaApple struct{}
type ChinaBanana struct{}
type ChinaPeach struct{}

func (ch *ChinaApple) EatApple() {
	fmt.Println("中国苹果")
}
func (ch *ChinaBanana) EatBanana() {
	fmt.Println("中国香蕉")
}
func (ch *ChinaPeach) EatPeach() {
	fmt.Println("中国🍑")
}

type ChinaFactory struct{}

func (ch *ChinaFactory) CreateApple() AppleAbs {
	var apple AppleAbs = new(ChinaApple)
	return apple
}

func (ch *ChinaFactory) CreateBanana() BananaAbs {
	var banana BananaAbs = new(ChinaBanana)
	return banana
}

func (ch *ChinaFactory) CreatePeach() PeachAbs {
	var peach PeachAbs = new(ChinaPeach)
	return peach
}

// 美国的产品生产线
type UsApple struct{}
type UsBanana struct{}
type UsPeach struct{}

func (ch *UsApple) EatApple() {
	fmt.Println("美国苹果")
}
func (ch *UsBanana) EatBanana() {
	fmt.Println("美国香蕉")
}
func (ch *UsPeach) EatPeach() {
	fmt.Println("美国🍑")
}

type UsFactory struct{}

func (ch *UsFactory) CreateApple() AppleAbs {
	var apple AppleAbs = new(UsApple)
	return apple
}

func (ch *UsFactory) CreateBanana() BananaAbs {
	var banana BananaAbs = new(UsBanana)
	return banana
}

func (ch *UsFactory) CreatePeach() PeachAbs {
	var peach PeachAbs = new(UsPeach)
	return peach
}

func main() {
	// 生成中国苹果
	var apple AppleAbs = new(ChinaApple)
	apple.EatApple()
	// 生成中国香蕉
	var banana BananaAbs = new(ChinaBanana)
	banana.EatBanana()
	// 生成美国苹果
	var apple1 AppleAbs = new(UsApple)
	apple1.EatApple()
	// 生成美国香蕉
	var banana1 BananaAbs = new(UsBanana)
	banana1.EatBanana()
	// 生成美国桃子
	var peach PeachAbs = new(UsPeach)
	peach.EatPeach()
}
