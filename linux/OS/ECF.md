# 概览

异常控制流 ECF, Exceptional Control Flow；术语方面有些人将异常叫做`中断`

想象下一个按顺序执行的程序，需要它停下来接受输入、出现异常时进入异常处理、开启新的运行副本等，一条能跳出程序顺序执行流程的操作，他们是怎样做到的

- 对于操作系统，实现 IO、进程、虚拟内存，都依赖于 ECF 为基础
- 对于并发来说，异常处理、时间重叠上执行进程、信号都依赖 ECF
- 对于编程语言想要实现，try catch throw 都离不开 ECF，非本地跳转
- ...各个层次都存在不同形式的 ECF

## 异常

由操作系统和硬件配合实现

1. 异常表
2. 执行异常处理程序
3. 最后异常处理程序触发以下情况之一：

- 返回给 CS 指针，发生异常的那条指令，然后继续执行
- 返回，发生异常那条指令的下一条指令
- exit

## 异常号

硬件设计者分配的：

- 除数为 0
- 算数运算溢出
- 缺页
- 内存访问越界
- 断点

操作系统开发者分配的：

- 系统调用
- 外围设备 IO 信号

## 异常表

存放异常号(索引)和与其对应的异常处理程序

表地址,存放在 CPU 的一个寄存器中，`异常表基地址寄存器`(exceptional table base register)

## 异常处理程序

调用异常处理程序，和常规的例程调用类似

1. 入栈用于返回的地址，根据不同异常类型，保存当前或者下一指令地址
2. 当前例程的运行状态入栈，flags, ...

这里这些例程运行在内核模式下，所以使用的是`内核栈`，0 特权级，操作系统资源访问权限

# 异常类型

## 异常类型-中断

(IO 设备信号触发，异步，返回到下一条指令)

按下键盘，中断引脚电压变高，CPU 控制权转移给中断处理程序，执行完成，返回继续程序运行

## 异常类型-陷阱

(为了发起系统调用故意为之，同步，返回到下一条指令)

用户通过 CPU 指令`syscall n`进行系统调用，从用户态切换到内核态；
使用这种指令就像是在调用一个自己写函数一样自然

## 异常类型-故障

例如缺页，内核引起，中断处理程序可能修复它，如果成功修复返回引发错误的指令重新执行，否则执行 abort 例程，它会终止引发异常的程序

## 异常类型-终止

不可恢复的致命错误，通常是硬件错误，直接执行 abort 例程，终止程序

## 异常与进程

----------------- 延伸 -----------------

### 上下文

由一切程序需要正确运行所需要的状态组成：

- 程序代码
- 数据
- 栈、堆
- flags 寄存器的内容
- 程序计数器
- 环境变量
- 打开文件描述符集合
- 虚拟内存页表
- 进程表

> 操作系统使用一种被称作`上下文切换`的异常控制流实现多任务，也就是多个进程

发生上下文切换的一些场景：

- 系统调用
- 内核调度发送抢占
- CPU 时钟到时引起中断
- IO 信号；例如磁盘读取完成信号，表示之前的挂起程序可以继续执行了

### 系统调用

实际上编程是应该都会使用 c 语言的包装的`系统调用函数`，这样更方便

到系统调用函数的参数都是通过寄存器传递的， 而不是栈:

- rax: 系统调用号
- rdi,rsi,rdx,r10,r8,r9: 以此顺序传递最多 6 个系统调用参数
- rax: 返回值