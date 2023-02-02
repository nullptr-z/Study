
# ioctl
`int ioctl(int fd, int request, .../* argp */)`
该系统调用为文件和设备操作提供了一种多用途机制;对于未纳入标注I/O模型的设备和文件操作,ioctl就是一个百宝箱
* fd: 已打开文件描述符
* request: 指定在fd上将执行的控制操作,具体设备的头文件定义了可传给*request*参数的常量
* argp: 需要根据*request*值来确定argp所期望的类型;通常情况下,argp都是整数或结构的指针,有些情况不需要argp参数

---

# fcntl
`int fcntl(int fd, int cmd, ...)`
内核会根据`cmd`的值来确认`...`参数的数据类型
fcntl用于获取(F_GETFL)或修改(F_SETFL)已打开文件的访问模式和状态标志(open的flag参数设置的)

### 获取状态:F_GETFL
传给cmd参数`F_GETFL`来获取:
```c
int flags = fcntl(fd, F_GETFL)
```

通常使用`&`符号来进行测试,例:
```c
if(flags & O_SYNC) // 测试是否已同步写方式打开
```

文件访问模式O_RDONLY(0), O_WRONLY(1), O_RDWR(2)比较特殊,可以看出他们并没有和打开文件状态标志单比特对应,使用掩码O_ACCMODE:
```C
int accMode = flags & O_ACCMODE
if(accMode == O_RDWR)
```

### 修改状态:F_SETFL
标准的Linux允许修改的标志有(有些unix还允许一些其他的O_SYNC...):
  * O_ACCMODE
  * O_NONBLOCK
  * O_NOATIME
  * O_ASYNC
  * O_DIRECT

修改文件状态使用场景:
  1) 文件描述符来自其他程序,自己需要修改其标志
  2) 文件描述符是用open之外的系统调获取的(无法设置标志)
```c
int flags = fcntl(fd, F_GETFL)
flags |= O_ACCMODE
fcntl(fd, F_SETFL,flags)
```

### 复制文件描述符
```3
fd = fcntl(fd, F_DUPFD)
fd = fcntl(fd, F_DUPFD, newfd)
```
* F_DUPFD: 用于表示复制fd描述符创建副本描述符 --类似dup(fd)
* newfd: 使用指定描述符newfd创建副本描述符,如果newfd已存在则向上增长  --类似dup(fd, newfd)
---

# 复制文件描述符
`dup (int oldfd)`
创建一个 oldfd文件描述符的副本,使他们指向同一个打开文件句柄

`dup2 (int oldfd, int newfd)`
使用 newfd指定的符号创建 oldfd文件描述符副本,使他们指向同一个打开文件句柄
  * 如果 newfd已存在,则会先强制关闭,且忽略关闭期间遇到的任何错误;更为安全的方法是事先自己手动`close(fd)`描述符
  * 如果 newfd和 oldfd相同则直接返回 oldfd,即不关闭也不创建
  * 如果 oldfd无效,返回错误号:EBADF

`dup3 (int oldfd, int newfd, int flags)`
基于dup2增加一个flags参数,修改系统调用行为的位掩码
  * O_CLOEXEC参数:促使内核为副本描述符设置close-on-exec标志(FD_CLOEXEC)

### shell的重定向功能
2>&1  将文件描述符2重定向到描述符1, 也就是吧标准错误描述符改为了标准输入描述符
---

# pread pwrite
`ssize_t pread/pwrite(int fd, void *buf, size_t count, off_t offset)`
他们完成的工作和 read/write类似,甚至更多,他们的操作都是从偏移 offset处开始的(基于文件头部),且不会改变文件的偏移
这两个函数完成的工作类似如下:
```c
  off_t orig = lseek(fd, 0, SEEK_CUR)
  lseek(fd, offset, SEEK_SET)
  read(fd, buf, len)
  lseek(fd, orig,SEEK_SET)
```
在多线程情况下,所有线程共享进程的文件描述符表,也就是说他们共享文件的偏移,以上代码非原子级操作,所以会引发竞争状态;

##### 优势和劣势:
  1) 执行单个 pread/pwrite系统调用所花时间比,多次调用lseek和read/write来达到目的的效率更高
  2) 实际pread/pwrite执行的I/O的效率是远低于read/write的
  3) 执行I/O的开销要远大于系统调用的,所以减少系统调用带来的优势也显得有限

---

# readv分散输入 writev集中输出
`ssize_t reacv/writev(int fd, const struct iovec *iov, int iovcnt)`
  * iov 缓冲区集合,每一个元素都是一个 iovec结构
  * iovcnt 缓冲区的个数

iovec结构:
```c
struct iovec {
  void *iov_base; // 指向buffer起始地址
  size_t iov_len;
}
```
  * iov_base: 用于设置指向缓冲区,可以任意数据类型,例如:char [], int, struct等; --在使用read/write时通常定义`char [BUFSIZ]`作为缓冲区
  * iov_len: 用于设置缓冲区的大小,以字节为单位sizeof(*\*iov_base*);标记iov_base指向结构的长度,作用和`BUFSIZ`一样

实际上通过循环多次进行read/write调用达到目的,但是_v版本系统调用效率更高,并且保证操作原子性

也可以分配一个大的缓冲区,再进行一次性I/O操作;这样也能保证原子性,但是这个大的缓存区需要在用户空间进行分配,多了一次复制操作,麻烦还降低了效率

# preadv pwritev
他们功能分别对应 pread pwrite

---

# 截断文件:truncate ftruncate
```c
tint runcate(char *pathname, off_t length)
int ftruncate(int fd, off_t length)
```
 * runcate: 操作的是路径,需要对文件有写权限,如果文件为符号链接,会自动进行解引用
 * fruncate: 需要open这类函数打开的文件描述符,并且写方式打开的;该调用不会改变文件偏移;

扩展文件:如果文件大于长度 length,将丢弃超出部分,如果小于 length,则使用空字节填充或者添加文件空洞
length大于文件长度时:
  1) fruncate调用:SUSv3标准要求返回错误或者扩展文件; XSI标准要求是扩展文件
  2) runcate:必须扩展文件

---

# 非阻塞I/O
在打开文件时指定O_NONBLOCK标志
  * 如果open没能立即打开文件,则返回错误,而非陷入阻塞;有一种情况例外,open操作FIFO时任可能陷入阻塞
  * 使用非阻塞代开的文件,后续I/O操作也是非阻塞的. 如果I/O没能立即完成,则可能只会传输部分数据,或者系统调用失败
  * 调用失败后返回EAGAIN或EWOULDBLOCK错误,他们在大多数发行版上表示的意思都一样

管道、FIFO、套接字、中端、伪终端设备都支持非阻塞模式。有些无法通过open进行标志设置,例如管道、套接字,这样的情况只能使用fcntl()的F_SETFL来设置

**普通文件不会陷入阻塞**,所以打开普通文件时通常会忽略O_NONBLOCK;然而当使用文件锁时也是有效的

---

# /dev/fd目录
每个进程都对应一个`/dev/fd/_n/`目录,_n代表着进程中文件描述符对应的编号。例如/dev/fn/0 代表进程的标准输入。

该目录实际是一个符号链接,链接到Linu特有的/proc/self/fd目录,这个目录又是Linux特有的/proc/PID/fd目录族之一,目录族每中一个目录符号链接都与某一进程中打开文件对应的

打开该目录下的文件就相当于复制响应的文件描述符;一下代码等价
```c
fd=open(/dev/fn/0, O_RDONLY)
fd=dup(0)
```
注意:设置这类文件访问模式,任何与原本不相同的访问模式都会被忽略,所以需要设置相同的访问模式

我了方便系统提供了/dev/stdin、stdout、stderr对应/dev/fd/0、1、2的符号链接

---

# 创建临时文件
C语言针对不同的系统,提供了一系列创建临时文件的库函数

### mkstemp
`int mkstemp(char *template)`
创建一个名称唯一的临时文件,以读写方式打开,并使用O_EXCL标志
本函数参数使用路径名形式,最后6个字符必须为`XXXXXX`,这6个字符会被替换以确保唯一;
不再使用时调用unlink(template)来删除临时文件

### tmpfile
`FILE *tmpfile(void)`
创建一个名称唯一的临时文件,以读写方式打开,并使用O_EXCL标志
该函数返回一个**文件流**供stdio库函数使用,文件流关闭后自动删除临时文件; 也就是该函数内部调用unlink()来删除的




