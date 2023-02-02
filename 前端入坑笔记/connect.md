
<Provider store={}/> 这个组件通常在最外层.所有的后代组件都能访问他的store

connect(state,action)(IU组件):
connect这个函数将参数传递给Provider组件的store,并返回一个对象,把对象以属性的方式传给UI组件
getState()获取当前的state
dispatch(action)

**connect干了些什么:**
connect是一个高阶函数，首先传入mapStateToProps、mapDispatchToProps，然后返回一个生产Component的函数(wrapWithConnect)，然后再将真正的Component作为参数传入wrapWithConnect，这样就生产出一个经过包裹的Connect组件，该组件具有如下特点:

通过props.store获取祖先Component的store

props包括stateProps、dispatchProps、parentProps,合并在一起得到newState，作为props传给真正的Component

componentDidMount时，添加事件this.store.subscribe(this.handleChange)，实现页面交互

shouldComponentUpdate时判断是否有避免进行渲染，提升页面性能，并得到newState

componentWillUnmount时移除注册的事件this.handleChange


##主要逻辑源码:
```
export default function connect(mapStateToProps, mapDispatchToProps, mergeProps, options = {}) {
  return function wrapWithConnect(WrappedComponent) {
    class Connect extends Component {
      constructor(props, context) {
        // 从祖先Component处获得store
        this.store = props.store || context.store
        this.stateProps = computeStateProps(this.store, props)
        this.dispatchProps = computeDispatchProps(this.store, props)
        this.state = { storeState: null }
        // 对stateProps、dispatchProps、parentProps进行合并
        this.updateState()
      }
      shouldComponentUpdate(nextProps, nextState) {
        // 进行判断，当数据发生改变时，Component重新渲染
        if (propsChanged || mapStateProducedChange || dispatchPropsChanged) {
          this.updateState(nextProps)
            return true
          }
        }
        componentDidMount() {
          // 改变Component的state
          this.store.subscribe(() = {
            this.setState({
              storeState: this.store.getState()
            })
          })
        }
        render() {
          // 生成包裹组件Connect
          return (
            <WrappedComponent {...this.nextState} />
          )
        }
      }
      Connect.contextTypes = {
        store: storeShape
      }
      return Connect;
    }
  }
```
