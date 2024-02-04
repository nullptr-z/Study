// 基于销售商额外做一些事情，海关、真伪..
// 场景，想海购商品，只继续告诉代理商；代理商从什么地方购买不需要关系，也不用操心海关、假货问题

package main

import "fmt"

type Goods struct {
	Name string // 种类
	Fact bool   // 真伪
}

// 商品抽象
type Shopping interface {
	Buy(goods *Goods)
}

// 商品销售商
type KoreaShopping struct{}   // 韩国销售商
type AmericaShopping struct{} // 美国销售商
type ChinaShopping struct{}   // 中国销售商

func (k *KoreaShopping) Buy(goods *Goods) {
	fmt.Println("韩国购买了", goods.Name)
}
func (k *AmericaShopping) Buy(goods *Goods) {
	fmt.Println("美国购买了", goods.Name)
}
func (k *ChinaShopping) Buy(goods *Goods) {
	fmt.Println("中国购买了", goods.Name)
}

// 代理
type Proxy struct {
	shopping Shopping
}

func NewProxy(sp Shopping) Shopping {
	return &Proxy{shopping: sp}
}

func (p *Proxy) Buy(goods *Goods) {
	// 真伪
	if !goods.Fact {
		fmt.Println("假货别买")
		return
	}
	// 找真正的销售商购买
	p.shopping.Buy(goods)
	// 海关
	fmt.Println(fmt.Sprintf("%s已过安检", goods.Name))
}

func main() {
	pp := &Goods{Name: "乒乓球", Fact: false}
	bk := &Goods{Name: "篮球", Fact: true}
	ymq := &Goods{Name: "羽毛球", Fact: false}

	var proxy = NewProxy(new(KoreaShopping))
	proxy.Buy(pp)

	proxy = NewProxy(new(ChinaShopping))
	proxy.Buy(bk)

	proxy = NewProxy(new(AmericaShopping))
	proxy.Buy(ymq)

}
