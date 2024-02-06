// 装饰器，包装类

package main

import "fmt"

// 手机类抽象
type Phone interface {
	Show()
}

// 手机装饰器类抽象
type PhoneDecorator interface {
	Phone
	Decorator(p Phone) PhoneDecorator
}

type Iphone struct{}

func (p *Iphone) Show() {
	fmt.Println("the iphone")
}

type HuaWei struct{}

func (w *HuaWei) Show() {
	fmt.Println("the huawei")
}

type Membrane struct {
	p Phone
	PhoneDecorator
}

func (w *Membrane) Decorator(p Phone) PhoneDecorator {
	w.p = p
	return w
}

func (w *Membrane) Show() {
	fmt.Print("make membrane of ")
	w.p.Show()
}

type Shell struct {
	p Phone
	PhoneDecorator
}

func (s *Shell) Decorator(p Phone) PhoneDecorator {
	s.p = p
	return s
}

func (s *Shell) Show() {
	fmt.Print("make shell of ")
	s.p.Show()
}

func main() {
	var huawei = new(HuaWei)
	// 裸机华为
	huawei.Show()
	var iphone = new(Iphone)
	// 裸机iphone
	iphone.Show()

	var dt PhoneDecorator = new(Membrane)
	// 贴膜的华为手机
	meHuawei := dt.Decorator(huawei)
	meHuawei.Show()
	// 贴膜的苹果手机
	meIphone := dt.Decorator(iphone)
	meIphone.Show()

	var shell PhoneDecorator = new(Shell)
	// 带壳又贴膜的华为手机
	meHuawei = shell.Decorator(meHuawei)
	meHuawei.Show()
	// 带壳又贴膜的苹果手机
	meIphone = shell.Decorator(meIphone)
	meIphone.Show()
}
