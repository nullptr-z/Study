JSX为React提供一个语法糖，简化React语法，例如：
{<div name="abc">123</div>} 
替换为以下React语法
React.createElement("div",{name:"abc"},123)

### 运行时选择组件
在JSX中如果要用一个小写开头的变量以组件的方式使用时,在使用前必须给他赋值一个大写开头的变量:
const components = {
  return <SpecificStory story={props.story} />
}

### 子代
标签之间的内容使用 props.children 传递

子代可以是任何内容,只要在该组件被渲染前能被转换成React 识别的东西:
{(index) => <div key={index}>This is item {index} in the list</div>}

{bler && <Header />} //只有 belr 为 true 时才会渲染 Header

String(value)   //转换为字符串
