## createStore简化版:
```
const createStore = (reducer) => {              接受一个reducer函数指针
  let state;                                    保存最新的state,通过store.getState()函数获取
  let listeners = [];                           保存监听函数,通过store.subscribe()函数设置

  const getState = () => state;                 返回state

  const dispatch = (action) => {                接受action
    state = reducer(state, action);             调用reducer更新state
    listeners.forEach(listener => listener());  自动执行监听函数
  };

  const subscribe = (listener) => {             接受监听函数指针
    listeners.push(listener);                   保存监听函数
    return () => {                              返回用于销毁监听函数的函数
      listeners = listeners.filter(l => l !== listener);
    }
  };

  dispatch({});                                 初始化

  return { getState, dispatch, subscribe };     返回Store
};
```