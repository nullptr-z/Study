## fflush(fd)
刷新标准输出缓冲区
`fflush(stdout) 把输出缓冲区里的东西打印到标准输出设备上`

在linux系统里面，一般都是行刷新，也就是要输出的内容会先放在缓冲区里面，直到遇到换行符或者缓冲区满了，才会将缓冲区里的内容全部输出到屏幕或者文件中

```c
  printf("abc");
  while (1);
```
这条语句并不会输出

纠正
```c
  printf("abc");
  fflush(stdout);
  while (1);
```
