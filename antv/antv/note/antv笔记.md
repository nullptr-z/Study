## 基类
 X6 的 Shape 命名空间中内置了一些基础图形，如 Rect、Edge、Circle 等，这些图形最终都有共同的基类 Cell，定义了节点和边共同属性和方法，如属性样式、可见性、业务数据等，并且在实例化、定制样式、配置默认选项等方面具有相同的行为

                                      ┌──────────────────┐
                                 ┌──▶│ Shape.Rect       │
                                 │   └──────────────────┘
                                 │   ┌──────────────────┐
                                 ├──▶│ Shape.Circle     │
                     ┌────────┐  │   └──────────────────┘
                  ┌─▶│  Node  │──┤   ┌──────────────────┐
                  │  └────────┘  ├──▶│ Shape.Ellipse    │
                  │              │   └──────────────────┘
                  │              │   ┌──────────────────┐
                  │              └──▶│ Shape.Xxx...     │
      ┌────────┐  │                  └──────────────────┘
      │  Cell  │──┤                                      
      └────────┘  │                  ┌──────────────────┐
                  │              ┌──▶│ Shape.Edge       │
                  │              │   └──────────────────┘
                  │  ┌────────┐  │   ┌──────────────────┐
                  └─▶│  Edge  │──┼──▶│ Shape.DoubleEdge │
                     └────────┘  │   └──────────────────┘
                                 │   ┌──────────────────┐
                                 └──▶│ Shape.ShadowEdge │
                                     └──────────────────┘
## Cell属性
选项名	类型	默认值	描述
id	String	undefined	节点/边的唯一标识，默认使用自动生成的 UUID。
markup	Markup	undefined	节点/边的 SVG/HTML 片段。
attrs	Object	{ }	节点/边属性样式。
shape	String	undefined	渲染节点/边的图形。
view	String	undefined	渲染节点/边的视图。
zIndex	Number	undefined	节点/边在画布中的层级，默认根据节点/边添加顺序自动确定。
visible	Boolean	true	节点/边是否可见。
parent	String	undefined	父节点。
children	String[]	undefined	子节点/边。
data	any	undefined	节点/边关联的业务数据。

## 节点支持的其他属性:
x	Number	0	节点位置 x 坐标，单位为 'px'。
y	Number	0	节点位置 y 坐标，单位为 'px'。
width	Number	1	节点宽度，单位为 'px'。
height	Number	1	节点高度，单位为 'px'。
angle

### 对一个已实例化节点进行修改
  ```js
* 实例化一个节点
const rect = new Shape.Rect()
* 修改这个节点
rect
  * 设置节点位置
  .position(100, 200)
  * 设置节点大小
  .resize(80, 40)
  * 旋转节点
  .rotate(30)
  * 设置节点样式
  .attr({
    body: {
      fill: 'blue',
    },
    label: {
      text: 'Hello',
      fill: 'white',
    },
  })
  * 添加到画布
graph.addNode(rect)
  ```


## 内置节点
构造函数	shape 名称	描述
Shape.Rect	rect	矩形。
Shape.Circle	circle	圆形。
Shape.Ellipse	ellipse	椭圆。
Shape.Polygon	polygon	多边形。
Shape.Polyline	polyline	折线。
Shape.Path	path	路径。
Shape.Image	image	图片。
Shape.HTML	html	HTML 节点，使用 foreignObject 渲染 HTML 片段。
Shape.TextBlock	text-block	文本节点，使用 foreignObject 渲染文本。
Shape.BorderedImage	image-bordered	带边框的图片。
Shape.EmbeddedImage	image-embedded	内嵌入矩形的图片。
Shape.InscribedImage	image-inscribed	内嵌入椭圆的图片。
Shape.Cylinder	cylinder	圆柱。

## 内置边
构造函数	shape 名称	描述
Shape.Edge	edge	边。
Shape.DoubleEdge	double-edge	双线边。
Shape.ShadowEdge	shadow-edge	阴影边。

### 内置边属性
```js
属性名	类型	默认值	描述
source	TerminalData	undefined	   源节点或起始点。
target	TerminalData	undefined	   目标节点或目标点。
vertices	Point.PointLike[]	undefined	路径点,按顺序经过路径点，最后到达target点。
router	RouterData	undefined	      路由,对路径点链接线进行处理。可选项 [normal,orth,oneSide,manhattan,metro,er],可自定义
connector	ConnectorData	undefined	  连线,路由处理的线转角进行类似圆滑的处理. 可选项 [normal,rounded,smooth,jumpover],可自定义
labels	Label[]	undefined	         标签。
defaultLabel	Label	                  默认标签,与后续的所有lable合并。
```
TerminalData可以是节点/桩的对象、ID,或者{x,y} [x,y]
```js
graph.addEdge({
  source: { cell: rect1, port: 'out-port-1' },  // 源节点和链接桩 ID
  target: { cell: 'rect2', port: 'in-port-1' }, // 目标节点 ID 和链接桩 ID
})

vertices: [
                { x: 100, y: 200 },
                { x: 300, y: 120 },
            ],
router: 'orth',
connector: "rounded"
```
```js
// 设置标签
edge.setLabels([{
  attrs: { label: { text: 'edge' } },
}])
// 或
edge.setLabels(['edge'])

// 添加单个标签
edge.appendLabel({
  attrs: { label: { text: 'edge' } },
})
// 或
edge.appendLabel('edge')

```
```js
attrs: {
                line: {
                    stroke: "#7c68fc", // 指定 path 元素的填充色
                    // 箭头选项 block,classic,diamond,cross,async,path,circle,circlePlus,ellipse, 可自定义svg箭头
                    sourceMarker: 'block', // 实心箭头
                    targetMarker: {       
                        name: 'ellipse', // 椭圆
                        rx: 10, // 椭圆箭头的 x 半径
                        ry: 6,  // 椭圆箭头的 y 半径
                    }
                },
            }
```



### 修改边的指向源/目标
edge
  .setSource(source)
  .setTarget(target)

## zIindex
cell.getZIndex() 和 cell.setZIndex(z: number) 来获取或设置 zIndex 的值，也可以调用 cell.toFront() 和 cell.toBack() 来将其移到最顶层或对底层。

## 指定默认配置
Shape.Rect.config(option)
每次调用都是将option与已有配置进的深度合并:只对指定的属性进行覆盖,其他属性保持不变,不会导致其他属性丢失