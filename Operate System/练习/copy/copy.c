#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>

int main(const int avgc, const char *avgs[])
{
  const char *sourceFileName1 = avgs[1];
  const char *directionFileName2 = avgs[2];
  int fd_source = open(sourceFileName1, O_RDONLY);
  int fd_dire = open(directionFileName2, O_WRONLY | O_CREAT | O_TRUNC, 0666);
  // if (fd_dire == -1)
  //   fd_dire = creat(directionFileName2, 0644);
  printf("fd_source: %d  fd_dire: %d\n", fd_source, fd_dire);
  char buff[BUFSIZ];
  int readCount = 0;
  while ((readCount = read(fd_source, &buff, BUFSIZ)) > 0)
  {
    printf("readCount: %d\n", readCount);
    int writeCount = write(fd_dire, &buff, readCount);
    if (writeCount != readCount)
      printf("写入文件遇到错误!");
    printf("writeCount: %d\n", writeCount);
  }

  // int writeCount = write(fd_source, buff, readCount);
  // printf("writeCount: %d\n", writeCount);

  return 0;
}