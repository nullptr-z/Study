Tarp: 系统调用 syscall，异常 exception，中断 interrupt；是这三种事件的统称

syscall：执行各种内核权限的任务
exception：处理所有用户空间的异常行为，甚至将其杀死
interrupt：响应各种外围设备 IO

## 重要的寄存器：

为了安全不能依赖于 32 User reg, 所有 Trap 时需要专有 reg

- STVEC：指向了内核中处理 trap 的指令的起始地址, 内核在这里写下 trap 处理程序的地址；RISC-V 跳转到这里来处理 trap。
-
- SEPC：当 trap 发生时，RISC-V 会将程序计数器(PC)保存在这里（因为 PC 会被 stvec 覆盖）。sret（从 trap 中返回）指令将 sepc 复制到 PC 中。内核可以写 sepc 来控制 sret 的返回到哪里。
-
- SCAUSE：RISC -V 在这里放了一个数字，描述了 trap 的原因。
-
- SSCRATCH：内核在这里放置了一个值，在 trap 处理程序开始时可以方便地使用。
-
- SSTATUS：SIE 位控制设备中断是否被启用，如果内核清除 SIE，RISC-V 将推迟设备中断，直到内核设置 SIE。SPP 位表示 trap 是来自 User mode 还是 supervisor mode，并控制 sret 返回到什么模式。

User 模式是不能使用的；matching 下处理的 trap，有一组等效的控制寄存器；xv6 只在定时器中断的特殊情况下使用它们。

多核芯片上的每个 CPU 都有自己的一组这些寄存器，而且在任何时候都可能有多个 CPU 在处理一个 trap。

> STVEC: Supervisor Trap Vector Base Address Register
> SEPC: Supervisor Exception Program Counter

## 执行步骤(时器中断特殊)：

> 1. 如果该 trap 是设备中断，且 sstatus SIE 位为 0，则不执行以下任何操作。
> 2. 通过清除 SIE 来禁用中断。
> 3. 复制 pc 到 sepc
> 4. 将当前模式（U 或 S）保存在 sstatus 的 SPP 位。
> 5. 在 scause 设置该次 trap 的原因。
> 6. 将模式转换为特权态。
> 7. 将 stvec 复制到 pc。
> 8. 从新的 pc 开始执行，就是 trap 处理程序。

## trap 处理程序必要的工作

> 1. 保存 32 个用户寄存器，恢复使用
> 2. 保存 PC 寄存器
> 3. 切换为 S mode
> 4. 切换 STAP 寄存器，让他指向 Kernel Page Table(从 UPT 切换过来)
> 5. 将 Stack Reg 指向内核空间
> 6. setting STVEC Reg, in trap

## S mode 的权利

除了读写以上寄存器

它还可以使用 PTE_U 标志位为 0 的 PTE

S model 能力也是非常有限的,任然需要通过 Page Table，受限于当前的虚拟地址， PTE_U = 1 页无法访问的
