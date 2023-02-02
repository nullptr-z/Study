#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/uio.h>
#include "readvs.c"


int main(int argc, char *argv[])
{
  if (argc >= 2)
  {
    int fd, x;
    struct iovec iov[3];
    struct stat myStruct;
#define STR_SIZE 20
    char str[STR_SIZE];

    ssize_t numRead, totRequied;

    fd = open(argv[1], O_RDONLY);
    if (fd == -1)
    {
      printf("打开文件失败");
    }

    // iov[0].iov_base = &myStruct;
    // iov[0].iov_len = sizeof(struct stat);
    // iov[1].iov_base = &x;
    // iov[1].iov_len = sizeof(x);
    iov[0].iov_base = str;
    iov[0].iov_len = STR_SIZE;
    iov[1].iov_base = str;
    iov[1].iov_len = STR_SIZE;
    iov[2].iov_base = str;
    iov[2].iov_len = STR_SIZE;

    totRequied = 0;
    totRequied += iov[0].iov_len;
    totRequied += iov[1].iov_len;
    totRequied += iov[2].iov_len;

    numRead = readvs(fd, iov, 3);
    if (numRead == -1)
    {
      printf("readv读取文件失败");
      return 0;
    }

    printf("请求读取字数:%ld,实际读取字数:%ld\n", totRequied, numRead);

    for (size_t i = 0; i < 3; i++)
    {
      write(1, iov[i].iov_base, iov[i].iov_len);
      putchar('\n');
    }
  }
  else
  {
    printf("请指定文件名");
  }

  return 0;
}
