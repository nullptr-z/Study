# 打开文件描述符表 > 系统级文件打开表 > 文件系统 i-node 表

他们都是由内核维护的,不过前者位进程级,后两者为系统级

## 打开文件描述符表(open file descriptor):

每个进行都拥有自己的表

- 控制文件描述符的一组标志(close-on-exec)
- 文件句柄的引用

### 系统级文件打开表(open file descriptor table): --也叫文件句柄表

将每一条目合称为**文件句柄**

- 当前文件的偏移量(调用 read,write,lseek 时改变)
- 文件的状态标志(open 的 flag 参数,O_CREAT,O_APPEND 等)
- 文件的访问模式(open 的 flag 参数: O_RDONLY,O_WRONLY,O_RDWR)
- 信号驱动 I/O 相关设置
- 对 i-node 的引用

### 文件系统 i-node 表:

所有文件信息都在该表中

- 文件类型和权限
- 一个指针,指向该文件所持有锁的列表
- 文件的各种属性,文件大小与不同类型(修改,创建,最近访问等)操作相关的时间戳

<details>
<summary>文件描述符,文件句柄,文件系统i-node之间的关系</summary>

![avatar](./img_文件描述符>句柄>i-node解释.png)
![avatar](./img_文件描述符>句柄>i-node.png)

</details>

---

# open

`int open(char* fileName,int flags,mode_t mode)`

- open 打开成功,返回的一定是未使用文件描述符最小的一个
- 只用 flgs 设置了 O_CREAT 标志时 mode 参数才有用

### 标志: --文件标志.png

访问方式

- O_RDONLY 以只读方式打开文件 --0
- O_WRONLY 以只写方式打开文件 --1
- O_RDWR 以读写方式打开文件 --2

写权限额外参数：O_WRONLY | [OPTION]

- O_CREAT 如果文件不存在,则创建文件
- O_EXCL 确保文件检查和创建是同一原子操作
- O*APPEND 将文件偏移移动和写入操作纳入同一原子操作
  *不能使用 O*RDONLY | O_WRONLY 代替 O_RDWR,这三个标志不能同时存在,同一时只能使用其中一种*

### 创建文件

创建文件设置 open 函数 mode 参数，umask 函数用于设置 umask,创建文件的权限就变了 mode & ~umask
[创建文件权限掩码](./img_%E5%88%9B%E5%BB%BA%E6%96%87%E4%BB%B6%E6%9D%83%E9%99%90%E6%8E%A9%E7%A0%81.png)

### open 失败

errno 标号的含义(部分):

- EACCES: 通常对文件的权限是不足,无法以 flgs 所设置的权限访问,文件可能不存在但也无法创建
- EISDIR: 指定的文件是一个目录,但是试图对其进行写操作
- EMFILE: 打开文件数量已达到进程资源所设定上限
- ENFILE: 打开文件数量已达到系统允许上限
- ENOENT: 文件不存在而且没指定 O_CREAT,或者指定了但文件路径错误,符号链接指向不存在的文件
- EROFS: 对只读方式打开的文件进行写入
- ETXTBSY: 指定了一个可执行文件,且正在允许

---

# readdir 读取目录

```c
while ((dep = readdir(streamp)) != NULL)
{
  printf("%s\t", dep->d_name);
}
```

---

# read

`ssize_t read(int fd, void *buffer, size_t count)`

- fd: 已打开文件描述符
- buffer: 指定存放数据的地址,容量不应小于 count
- count: 读取数据字节数量
- ssize_t: 有符号类型,返回读取结果:`实际读取数量`或者`-1`
  注意 read 遇到`\0`并不会结束,而且也不会在读取的数据结尾加`\0`

---

# write

- fd: 通常为 open 的返回值
- buffer: 已打开文件描述符
- count: 写入数据字节数量,不应大于 buffer 的容量
- ssize_t: 有符号类型,返回读取结果:`实际写入数量`或者`-1`；有意思的是这种设计导致，reed 可读取的最大数量少了一半
  注意当写入成功情况,实际写入数量少数情况下会小于 count,例如磁盘满了或者被进程资源对文件大小的限制;
  write 调用成功并不代表文件已经写入磁盘,内核为了减少磁盘访问,会在内存中缓存磁盘的 I/O 操作

---

# close

`int close(int fd)`
虽然关闭进程时所有打开的文件会被自动关闭,但是随时关闭不再需要的文件是个好习惯,毕竟内存有限
检查关闭文件的返回错误状态:

- 指定未打开的文件描述符,对一个文件描述符多次关闭
- 在关闭 NFS(网络文件系统)时,如果 NFS 提交失败,代表着数据没有抵达远程磁盘

---

# lseek

`off_t lseek(int fd, off_t offset, int whence)`
内核会记录每个已打开文件的各种信息,例如**偏移量**,文件的第一个字节偏移量为 0
这正是 read()和 write()要进行读写的起始位置,读写完成后将偏移量置于新的位置

- fd: 已打开文件描述符
- offset: 以字节为单位,进行偏移的字节数量--off_t 为有符号整型
- whence: 设置偏移量的起点,使用如下常量
  1. SEEK_SET 从文件的头部开始
  2. SEEK_CUR 从文件当前位置开始
  3. SEEK_END 从文件结尾处开始
- 返回值: 返回文件新的偏移位置
- `lseek(fd, 0, SEEK_CUR)`返回当前位置, 有些 UNIX 中使用的是非标准函数 tell(fd)返回当前位置\*
- lseek 早期叫做 seek,offset 参数和返回值都是 int 类型,后来改为 long 随即改名为 lseek\*

lseek 函数并不能用于管道丶 FIFO 丶 socket 或终端

## stat

_include <sys/stat.h>_

```
int stat(const char *fileName, struct stat *stat)
int fstat(int fd, struct stat *stat)
```

获取文件的各种信息，保存到`stat`结构

### 文件空洞

即使偏移已达到结尾,write 任然可以文件尾后任意位置处继续写入，偏移指针继续移动，被跳过的部分就是文件空洞
从程序角度来看读取文件空洞任然是存在字节的,使用 0 填充内存;
实际上文件系统是没有为文件空洞分配磁盘块的,所以它是不占用磁盘空间的;--注意磁盘是以块为单位的,只有空洞字节数大于一个块的剩余字节数才造成真实的磁盘空间浪费

---
