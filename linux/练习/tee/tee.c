#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
// #include <../../C/tlpi-book/lib/tlpi_hdr.h>

int main(int argc, char *argv[])
{
  printf("EOF 等于 -1?:%s\n", EOF == -1 ? "YES" : "NO");
  if (argc >= 2)
  {
    const char *tarFile = argv[1];
    int tarfd, readCount = 0, wrResult = 0, lineCount = 0;
    int openFlags = O_WRONLY | O_CREAT | O_TRUNC;
    char buffer[BUFSIZ], line[20];

    char opt;
    while ((opt = getopt(argc, argv, "ab")) != -1)
      if (opt == 'a')
      {
        openFlags ^= O_TRUNC;
        openFlags |= O_APPEND;
      }

    tarfd = open(tarFile, openFlags, 0644);
    if (tarfd != -1)
    {
      while ((lineCount = read(STDIN_FILENO, &line, 20)))
      {
        if (lineCount == 0)
          break;
        if (lineCount == -1)
        {
          printf("读取标准输入遇到错误!\n");
          _Exit(errno);
        }
        printf("%s", line);
        for (int i = 0; i < lineCount; i++)
        {
          buffer[readCount + i] = line[i];
        }
        readCount += lineCount;
      }
      printf("增加: %d字节数据\n", readCount);

      wrResult = write(tarfd, &buffer, readCount);
      if (wrResult != readCount)
      {
        printf("向文件:%s 写入数据遭遇中途退出!\n", tarFile);
      }
    }
    else
    {
      printf("打开文件遇到错误!\n");
    }
  }
  else
  {
    printf("需要指定一个输出目标文件!\n");
  }

  return 0;
}
