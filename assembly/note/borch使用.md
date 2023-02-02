bximage 命令:创建磁盘镜像

```
dd if=boot.bin of=a.img bs=512 count=1 conv=notrunc
```
if:二进制代码
of:磁盘镜像,或者存储设备挂载的位置:devfd0
bs:扇区的长度
count:扇区数量
conv=notrunc:代表不截断磁盘镜像



