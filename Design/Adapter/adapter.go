// 适配器，将两个不相干的类的，适配一起工作，通过重写父类结构的实现做到这一点

package main

import "fmt"

type PowerOff interface {
	Power()
}

type PowerV220 struct{}

func (p *PowerV220) Power() {
	fmt.Println("220V 充电中")
}

type AdapterV5 struct {
	pf PowerOff
}

func (a *AdapterV5) PowerUseV5(p Phone) {
	// ...变压，v220 to v5
	a.pf.Power()
}

type Phone struct {
	name string
}

func (p *Phone) Power() {
	fmt.Println("给手机充电")
}

func main() {
	var phone = Phone{name: "锤子手机"}
	var ada = AdapterV5{pf: new(PowerV220)}
	ada.PowerUseV5(phone)
}
