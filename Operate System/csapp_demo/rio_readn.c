#include "csapp.h"
#include <sys/stat.h>
#include <dirent.h>

// ssize_t rio_readn(int fd, void *usrbuf, size_t n);
// ssize_t rio_readn(int fd, void *usrbuf, size_t n)
// {
//   size_t nleft = n;
//   ssize_t nread;
//   char *bufp = usrbuf;

//   while (nleft > 0)
//   {
//     if ((nread = read(fd, bufp, nleft)) < 0)
//     {
//       if (errno == EINTR) // 被信号处理器打断
//         nread = 0;
//       else
//         return -1;
//     }
//     else if (nread == 0)
//       break;
//     nleft -= nread;
//     bufp += nread;
//   }
//   return n - nleft;
// }

int main()
{
  int fd;
  char *buf[BUFSIZ], *type;
  fd = open("test_file", O_RDONLY, 0);
  // ssize_t n = rio_readn(fd, buf, 100);
  // printf("%d\n", n);

  struct stat stat;
  Stat("test_file", &stat);

  type = S_ISREG(stat.st_mode);
  type = S_ISSOCK(stat.st_mode);

  DIR *dir_ptr = opendir("/");

  // rio_readn(dir_ptr->__dd_fd, buf, 1000);
}
