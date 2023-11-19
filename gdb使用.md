`si` 单步执行一条机器指令
`rsi` 往回退一步
`s` 单步执行，如果当前断点是函数，会进入函数
`n` 执行到下一条指令，不会进入函数
`finish` 跳出当前函数
`p` 打印变量，还可以打印寄存器 p %eax
`i locals` 哪些变量可以打印查看
`c` 继续执行，会停在断点
`b label` 对标号(label，函数名)设置为断点

`layout [src | asm]` 显示源码窗口

`target remote` 链接远程调试

`fours src` 把光标移动到源码,方便查看代码上下文;Ctrl+C 退回 Command

`set scheduler-locking on` 暂停其他线程

`info threads` 查看线程信息,例如当前运行的线程 num

`thread num` 切换到 num 线程去执行

直接回车会执行上一次输入的命令

## 配置性操作

`~/.gdbinit` 用于配置一些初始运行的指令

```sh
directory PATH     # 配置静态库目录， 调试时就可以进入.c的文件，而不仅仅是一个头文件
layout src
set pagination off # 关闭confirm
```

.gdb 文件：
也用来写一些 gdb 指令, 第一行写要调试的二进制文件名,`gdb -x *.gdb`
