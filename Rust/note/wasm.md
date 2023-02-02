动画帧事件
requestAnimationFrame()

FPS取值算法
两次requestAnimationFrame动画帧事件调用的时间戳间隔diff
1 / diff * 1000

window.performance.now() 从界面加载，到现在的时间戳


```js 使用wasm将二进制码解析为javascript对象
WebAssembly.compile(new Uint8Array(`
  00 61 73 6d  01 00 00 00  01 0c 02 60  02 7f 7f 01
  7f 60 01 7f  01 7f 03 03  02 00 01 07  10 02 03 61
  64 64 00 00  06 73 71 75  61 72 65 00  01 0a 13 02
  08 00 20 00  20 01 6a 0f  0b 08 00 20  00 20 00 6c
  0f 0b`.trim().split(/[\s\r\n]+/g).map(str => parseInt(str, 16))
)).then(module => {
  const instance = new WebAssembly.Instance(module)
  const { add, square } = instance.exports

  console.log('4 + 8 =', add(4, 8))
  console.log('5^2 =', square(5))
  console.log('(4 + 8)^2 =', square(add(4 + 8)))

})
```
