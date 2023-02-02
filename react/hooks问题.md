 ## useEffect
 useEffect 卸载函数会造成闭包问题
 ```js
  useEffect(() => {
    return () => {
      console.log(count);
    }
  }, []);
```
注: useEffect 的建议：
只有变化时，需要重新执行 useEffect 的变量，才要放到 deps 中。而不是 useEffect 用到的变量都放到 deps 中。
在有延迟调用场景时，可以通过 ref 来解决闭包问题。


## useState
ahooks 的 useSetState类似this.setState,用于整合同类 useState的 state更新


## Ref

不能在函数组件上使用 `ref` 属性，因为函数组件没有实例

useRef 是 React16.8 中引入的，只能在函数组件中使用。

闭包问题都可以通过 useRef解决
useRef 保证任何时候访问的 countRef.current 都是最新的，以解决闭包问题
 = [state]
注:  countRef.current类似countRef.current于引用类型,保存变量的地址


