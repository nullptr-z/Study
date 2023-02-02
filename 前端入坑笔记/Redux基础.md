*安装*
npm install redux
npm install react-redux

**使用场景**
某个组件的状态，需要共享
某个状态需要在任何地方都可以拿到
一个组件需要改变全局状态
一个组件需要改变另一个组件的状态

**设计思想**
将整个Web应用看做一个状态机,视图(**view**)与状态(**state**)一 一对应
所有的状态保存在一个对象(**store**)里面

**Store**
--import {createStore } from 'redux'--
可以看做一个容器,整个应用只有一个store 对象. 它是Redux 的createStore() 函数 返回的一个Store 对象
const Store=createStore(fnu)   

Store 含有所有数据,如果想获取某一运行时刻的数据,可以使用 store.getState()方法,返回程序当前时刻的 state,state 是这个运行时刻的数据集
Redux中一个 state 对应一个view, 也就是说view 和 state 总是同时发生改变的,
如果一个web应用的用户只能对view进行操作,所以state发生了改变给变就说明用户进行了操作, 但是view在什么情况发出通知 改变state,是程序员决定的;--- view和 state 一定是互相发生改变的

import { init } from '@rematch/core'
init(option)函数是创建store的核心函数
option{
  plugins:['插件']
}

**Action**
const action = {    本质就是一个普通对象,没什么特殊
  type: '',         约定第一个字符串常量 type Ation的名称
  payload: ''       其他自定义参数
Ation把数据传递给Store来改变state 的数据,也是改变State 的唯一办法

View有多少种消息,就有多少种Action.通常使用这样的方式来生成多个Action:
const newTodo='NewTodo';
function addTodo(msg){
  return {
    type: newTodo,
    msg:msg,
  }
}
const action= addTodo('msg')
通常这种addTodo 叫做Action Creator

**dispatch()**
view发出 action的唯一方法,就是调用store.dispacth(action),传递给store
const store = createStore(fn);

store.dispatch({
  type: 'ADD_TODO',
  payload: 'Learn Redux'
});

**Reducer**1
Store接收到action后,将使用action来更改state,产生一个新的state,从而更新view,我们把这个改变过程叫做Reducer
这个函数接受state 和Action 作为参数,返回新的state
*//如果 state 有改动,则返回新的 state使 view刷新*
const reducer = ( state, action) => {  
    switch (ation.type){
      ...根据实际情况更改state
        return newState
    }
}

**实际应用时**,在创建 store 时就把reducer 传递给 
createStore(reducer ),此后每当调用 store.dispatch(action),就会自动调用 reduce,得到新的 state,使 view刷新

reducer是一个纯函数,为了保证同样的 state必定得到同样的 view,所以在 reducer 函数能不得改变 state,能把 state 设置为只读最好
createStore()还接受第二个可选参数,表示state的最初状态,这通常是服务器发出的,如果提供了这个参数,他会覆盖默认值:
createStore(todo, window.STATE_FROM_SERVER)
}
  

*import { Provider } from 'react-redux'*

let store = createStore(App);

<Provider store={store} >chrilden </Provider>

所有chirlden(子组件) 都能访问 state


**store.subscribe()**
store.subscribe(function),接受一个函数,一个监听函数,一旦 State 发生变化，就自动执行这个函数
它返回另一个无参函数,调用这个函数就会解除监听
--只要把 render 或者 setState 函数的调用放进这个函数就可以自动渲染view
