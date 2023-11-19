#include <sys/stat.h>
#include <fcntl.h>
#include <ctype.h>
#include "./lib/tlpi_hdr.h"

int main(int argc, char const *argv[])
{
  size_t len;
  off_t offset;
  int fd, ap;
  ssize_t readCount, writeCount;
  char *buf;
  if (argc < 3)
    usageErr("%s 命令需要至少3个参数", argv[0]);
  fd = open(argv[1], (O_RDWR | O_CREAT), 0666);
  if (fd == -1)
    errExit("open 返回状态 -1");
  for (ap = 2; ap < argc; ap++)
  {
    switch (argv[ap][0])
    {
    case 'r':
    case 'R':
      len = getLong(&argv[ap][1], GN_ANY_BASE, argv[ap]); // 将字符串转换位GN_ANY_BASE(十进制)进制的整数
      buf = (char *)malloc(len);
      if (buf == NULL)
        errExit("malloc 返回null");
      readCount = read(fd, buf, len);
      if (readCount == -1)
        errExit("read 返回 -1");
      else if (readCount == 0)
        printf("read:已读取到文件(%s)结尾", argv[1]);
      else
      {
        if (argv[ap][0] == 'r')
          for (size_t j = 0; j < readCount; j++)
            printf("%c", buf[j]);
        else
          for (size_t j = 0; j < readCount; j++)
            printf("%#x\t", buf[j]);
        printf("\n");
      }
      free(buf);
      break;
    case 'w':
      writeCount = write(fd, &argv[ap][1], strlen(&argv[ap][1]));
      if (writeCount == -1)
        errExit("write 返回 -1");
      printf("写入%s到文件%s\n", &argv[ap][1], argv[1]);
      break;
    case 's':
      offset = getLong(&argv[ap][1], GN_ANY_BASE, argv[ap]);
      if (lseek(fd, offset, SEEK_SET) == -1)
        errExit("lseek 返回 =1");
      printf("文件%s 偏移%lld\n", argv[1], offset);
      break;
    default:
      break;
    }
  }
  close(fd);
  return 0;
}
