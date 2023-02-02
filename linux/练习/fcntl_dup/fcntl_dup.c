#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

int dup_z(int fd)
{
  int fdtmp = fcntl(fd, F_DUPFD);
  return fdtmp;
}

int dup2_z(int fd, int newfd)
{
  int flags, fdtmp;

  flags = fcntl(fd, F_GETFL);
  if (flags != -1)
  {
    if (fd != newfd)
    {
      fdtmp = fcntl(newfd, F_GETFL);
      if (fdtmp != -1)
      {
        printf("指定 newfd:%d已存在,将强制关闭!\n", newfd);
        close(newfd);
      }else
      {
        printf("newfd:%d未使用\n", newfd);
      }

      fdtmp = fcntl(fd, F_DUPFD, newfd);

      return fdtmp;
    }
    else{
      printf("原描述符fd:%d 与 指定newfd:%d相同!\n", fd, newfd);
      return fd;
    }
  }
  else
  {
    printf("目标文件不存在");
  }

  return flags;
}

int main(int argc, char const *argv[])
{
  int fd, newfd = argv[2][0] - '0';

  fd = open(argv[1], O_RDWR);
  printf("指定新描述符描述符 fd:%d\n", newfd);
  dup_z(fd);
  fd = dup2_z(fd, newfd);
  printf("获得新描述符 fd:%d\n", fd);
  int wRuslt = write(fd, "123", 4);
  printf("测试写入字符数量:%d\n", wRuslt);
  return 0;
}
