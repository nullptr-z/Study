因为是弱类型语言，这可以这样使用：
短路求值：如果条件是 true，&& 右侧的元素就会被渲染，如果是 false，React 会忽略并跳过它。
三元运算符可能需要把值（）括起来

preventDefault() //阻止ULR跳转

toUpperCase() //强转为大写

e.target是指点击时的元素，this指的是当前元素，如果该元素有内部节点，this会选择子节点，这时可以用target
---

以下函数都可以对字符串做转换，如果能转化为数字则当数字处理
Number.parseFloat("12") 转为浮点数

Number.parseInt("12.10") 转为整数

Number.isInteger(121.0);判断一个值是否为整数

Number.isNaN(); 检查一个值是否为NaN

Math.trunc()：去除一个数的小数部分，

Math.sign()：判断一个数到底是正数、负数、还是零

isFinite() 函数用于检查其参数是否是无穷大。如果 number 是有限数字（或可转换为有限数字），那么返回 true。否则，如果 number 是 NaN（非数字），或者是正、负无穷大的数，则返回 false。

array1.concat(array2,array3,...) 拼接数组,返回拼接后的新数组

map() 遍历打印

.bind(this)  绑定当前即将被销毁的对象

lambda 表达式中需要创建对象是要用（{}）小括号把大括号括起来

## 扩展字符
...参数名，主要用于函数参数列表最后，获取函数多余的参数，或者将数组转化为以逗号隔开的参数列表，不能使用argument对象
[..."abc"] //还能把字符串拆为字符数组a,b,c
{ kind, ...other } = props //第一个值赋值给 kind,其他参数都在 other

---
## this
以new的方式来调用：函数内部this指向本次函数执行时对应的一个匿名对象。

通过call的方法来间接调用函数：函数内部this指向call函数的第一个形参（自己指定this）

函数中this总是指向函数调用者，如果是直接使用函数则指向window，箭头函数则总是指向最外层


# 导入_导出常量、函数、文件、模块等
### import 
import {a} from 'XXX'
导入XXX文件中的a组件

import {a as b} from 'XXX'
导入XXX文件中的a组件，并将其重命名为b

import * as a from 'XXX'
导入XXX文件中的所有组件，并将其命名为a，调用具体组件的方式为a.b、a.c

### export
export{name,names} 括号是必须的
export default name 不需要括号但, 在整个文件中只能有一个,且只能传单个数据


## obj
Object.keys() 方法会返回一个由一个给定对象的自身可枚举属性组成的数组,使用Key作为这个枚举属性 而不是值

Object.getOwnPropertyNames() 只返回不可枚举对象

## for_in 与 for_of
for(car value in array)     //不会输出,undefind,使用 obj.key作为索引
if(hasOwnProperty.call(array,value))  //过滤掉用于枚举的属性,只取实例

for(car value of array) //输出所有对象值
**for in 会获取可枚举值(通常是 key),进行排除 undefined 对象,而 for..of 输出所有元素,效率略高,但是会给出空值错误**

const obj = ()=>{ }  这里的 obj 是一个函数,因为给他赋值的是一个匿名函数,obj 拥有和这个匿名函数相同的参数个数;obj是一个函数表达式





