#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

#define BUFSIZ 5


int main(const int avgc, const char *avgs[])
{
  if (avgc == 2)
  {
    int fd;
    off_t offset;
    ssize_t readResult, writeResult;
    char readBuffer[BUFSIZ];

    fd = open(avgs[1], O_RDWR | O_APPEND);
    if (fd != -1)
    {
      offset = fcntl(fd, 0, SEEK_SET);

      readResult = read(0, readBuffer, BUFSIZ);
      writeResult = write(fd, readBuffer, BUFSIZ);
      if (readResult)
      {
        printf("写入:%zd 个字符", writeResult);
      }
      
    }
  }
  else
  {
    printf("参数有误:%d", avgc);
  }
}