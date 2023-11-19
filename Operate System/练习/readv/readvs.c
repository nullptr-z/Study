#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/uio.h>

int readvs(int fd, struct iovec iov[], int iovCount)
{
  int numRead, count;

  // 无法保证原子操作
  // int flags = fcntl(fd, F_GETFL);
  // printf("flags:%d\n\n", flags);
  // flags |= O_EXCL;
  // printf("flags:%d\n\n", flags);
  // int set = fcntl(fd, F_SETFL, flags);
  // printf("set:%d\n\n", set);

  // flags = fcntl(fd, F_GETFL);
  // printf("flags:%d\n\n", flags);

  for (size_t i = 0; i < iovCount; i++)
  {
    numRead = read(fd, iov[i].iov_base, iov[i].iov_len);
    if (numRead == -1)
    {
      printf("read读取错误！");
      return numRead;
    }
    count += numRead;
  }
  return count;
}
