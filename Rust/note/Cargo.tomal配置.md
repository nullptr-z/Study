当一个引入一个 create，他声明过的 Re-exports,那么在使用这个 create 的时候，就不需在当前项目中引入这些被 Re-exports 的 create

## use feature flag

可选的特性，在封装 create 时，有些特性 n 可以让用户决定是否启用

## dev and release

指定开发环境和发布环境的构建行为

- panic = "abort" 代替默认的 panic! 宏，panic! 宏会在 panic 时打印出错误信息，然后退出程序，而 abort 会直接退出程序，不会打印错误信息

```sh
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```
