
#include <stdio.h>
#include <unistd.h>

int getch(int fd)
{
  static char str[BUFSIZ];
  static char *strPrt = str;
  static int n = 0;
  if (n == 0)
  {
    n = read(fd, str, BUFSIZ);
    strPrt = str;
  }
  return --n >= 0 ? (unsigned char)*strPrt++ : EOF;
}

int getchs(char *str, const unsigned max, int fd)
{
  unsigned n = 0;
  char c;
  while ((c = getch(fd)) != EOF && n < max - 1)
  {
    str[n++] = c;
  }
  str[n] = '\0';
  return n;
}
