#include <unistd.h>
#include <fcntl.h>

int main(int argc, char const *argv[])
{
  int fd1, fd2, fd3;
  const char *file = argv[1];
  fd1 = open(file, O_RDWR | O_CREAT | O_TRUNC, 0420);
  fd2 = dup(fd1);
  fd3 = open(file, O_RDWR| O_TRUNC);
  write(fd1, "Hello,", 6);
  write(fd2, " world", 6);

  lseek(fd2, 0, SEEK_SET);
  write(fd1, "HELLO,", 6);
  write(fd3, "Gidday", 6);
  return 0;
}
