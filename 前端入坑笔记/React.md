withRouter方法目的就是让非路由组件可以从属性中获取 history,location,match,query,params,url,search 等路由信息的————以赋值的方式

## PropTypes开发模式属性验证
  *import PropTypes from 'prop-types'*
Greeting.propTypes = {    //PropsTypes用于开发期间保证接受数据的有效性
  name: PropTypes.string  //给属性传递了无效值时,控制台会给出警告
  boo:  PropTypes.string
};                        //效率不高,只在开发模式下生效

Greeting.defaultProps = {
  name: 'zhou'            //为 props 定义默认值
};

## PureComponent || Component
Component在渲染页面时通过生命周期中的 shouldComponentUpdate方法来判断页面是否需要重新渲染，默认该方法返回true --- 改变props或state 参数值需要重新渲染时使用

PureComponent在调用 shouldComponentUpdate方法前先进行浅比较,仅比较两者的内存地址是否相同，而对于其值是否发生变化，则不会理会,所以效率较高 --- 只有在props或state 参数数量变化的时候才会进行渲染

遵循下列两个简单的规则就可以安全的使用PureComponent来代替Component:

**- 虽然通常情况下易变性就是不好的，但是当使用`PureComponent`时问题会变得复杂。**
**- 如果你在`render`方法中创建一个新的函数，对象或者是数组那么你的做法（可能）是错误的。**

## React 在某个事件里注册函数访问,如果加括号(),则加载时会自动触发