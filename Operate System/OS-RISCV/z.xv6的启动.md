1. loading boot loader from ROM
2. loader will xv6 kernel in 0x80000000 of memory
3. entry: kernel/entry.S:6, set up stack for C language
4. jump to kernel/start.c:21; a series of initialize; focus :line 32 `w_mepc((uint64)main)`;run instruction mret
5. (supervisor mode) jump to kernel/main.c:11; a series initialize
6. call `userinit()` jump to kernel/proc.c:212, the is first process
7. run initcode user/initcode.S：exec("/init") user/init.c:15, open file description 0、1、2

========================================
0x1000 Boot ROM，这是 RISC-V 加电后就立即执行的地方，然后回跳转到 0x8000,我们需要做的就是把操作系统代码写入 0x8000

每个 CPU 都会到 main.c 执行一系列初始化工作，最后进入`scheduler()`无线循环，等待调度程序分配任务
