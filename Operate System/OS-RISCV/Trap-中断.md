## 启用中断

先对设置 SIE 寄存器表示接收三种中断，
设置 UART 启用中断，
设置 PLIC 中断控制器接受 UART，
最后调度器将每个 CPU 的中断使能寄存器开启

## 外部硬件

UART，全称为 Universal Asynchronous Receiver/Transmitter，是一种常见的串行通信协议，用于在计算机系统中通过串口（Serial Port）进行数据传输。一次读取一个字节的数据；这种模式被称为编程 I/O，简单，但速度太慢；DMA(direct memory access)需要高速移动大量数据的设备通常使用直接内存访问
0x10000000

CLINT: 定时器
0x20000000

## PLIC

PLIC（Platform-Level Interrupt Controller）是 CPU 上一个硬件组件，用于管理和分发来自不同设备的中断。PLIC 旨在支持多核处理器系统，确保中断能够有效地在多个处理核之间进行分发。
0x0C000000

- PLIC 会通知当前有一个待处理的中断
- 其中一个 CPU 核会 Claim 接收中断，这样 PLIC 就不会把中断发给其他的 CPU 处理
- CPU 核处理完中断之后，CPU 会通知 PLIC
- PLIC 将不再保存中断的信息

内核可以对中断优先级进行编程，告诉它中断应该分发到哪，注意只有一个 CPU 会 Claim 的中断

> 相关函数：
> main(): 每个 CPU 核心都会到这里执行一系列初始化
> plicinit(): 设置开启哪些硬件设备的中断
> plicinithart(): 每个 CPU 的核都通过这个函数来表明对于哪些外设中断感兴趣
> scheduler(): 首先调用`intr_on()`开启接受中断，进入空转，等待任务调度器中断，分配任务

## 驱动程序

通常情况下，驱动中会有一些队列（或者说 buffer），top 部分(调用者)的代码会从队列中读写数据，而 Interrupt handler（bottom 部分）同时也会向队列中读写数据

通常对于 Interrupt handler 来说存在一些限制，因为它并没有运行在任何进程的 context 中，所以进程的 page table 并不知道该从哪个地址读写数据，也就无法直接从 Interrupt handler 读写数据。驱动的 top 部分通常与用户的进程交互，并进行数据的读写。这是一个驱动的典型架构。

## 一个外部中断发生的过程

> 从 consoleread 函数中可以看出，当读指针和写指针一样时，说明 buffer 为空，进程会 sleep。所以 Shell 在打印完“$ ”之后，如果键盘没有输入，Shell 进程会 sleep，直到键盘有一个字符输入。所以在某个时间点，假设用户通过键盘输入了“l”，这会导致“l”被发送到主板上的 UART 芯片，产生中断之后再被 PLIC 路由到某个 CPU 核，之后会触发 devintr 函数，devintr 可以发现这是一个 UART 中断，然后通过 uartgetc 函数获取到相应的字符，之后再将字符传递给 consoleintr 函数。
