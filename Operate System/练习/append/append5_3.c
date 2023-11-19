#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>

int main(const int avgc, const char *avgs[])
{
  if (avgc >= 3)
  {
    int fd, len, isx;
    off_t offset;
    ssize_t readResult, writeResult;
    char *readBuffer = "c\n";

    len = atoi(avgs[2]);

    isx = avgc == 4 && *avgs[3] == 'x';

    if (isx)
    {
      fd = open(avgs[1], O_CREAT | O_RDWR, 0644);
    }
    else
    {
      fd = open(avgs[1], O_CREAT | O_RDWR | O_APPEND, 0644);
    }

    if (fd != -1)
    {
      printf("开始写入");
      for (size_t i = 0; i < len; i++)
      {
        if (isx)
        {
          offset = fcntl(fd, 0, SEEK_END);
        }

        writeResult = write(fd, readBuffer, 2);
        if (readResult)
        {
          printf("%zu\n", i);
        }
      }
    }
    else
    {
      printf("\n打开文件失败");
    }
  }
  else
  {
    printf("\n参数有误:%d", avgc);
  }
}