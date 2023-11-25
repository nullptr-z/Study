## 中断-硬件

UART，全称为 Universal Asynchronous Receiver/Transmitter，是一种常见的串行通信协议，用于在计算机系统中通过串口（Serial Port）进行数据传输。一次读取一个字节的数据；这种模式被称为编程 I/O，简单，但速度太慢；DMA(direct memory access)需要高速移动大量数据的设备通常使用直接内存访问
0x10000000

CLINT: 定时器
0x20000000

PLIC（Platform-Level Interrupt Controller）是一种硬件组件，通常用于处理器架构中，用于管理和分发中断。PLIC 旨在支持多核处理器系统，确保中断能够有效地在多个处理核之间进行分发和处理。
0x0C000000

## 启用中断

先对设置 SIE 寄存器表示接收三种中断，
设置 UART 启用中断，
设置 PLIC 中断控制器接受 UART，
最后调度器将每个 CPU 的中断使能寄存器开启
