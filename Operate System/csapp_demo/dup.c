#include "csapp.h"

int main()
{
  int fd1, fd2 = 99;
  char c;
  char *buf = "9999999";

  fd1 = Open("test.txt", O_RDWR, NULL);
  dup2(fd1, fd2);
  write(fd2, buf, sizeof buf);

  exit(0);
}
