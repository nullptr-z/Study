#include <unistd.h>

int putch(int c, int pd)
{
  size_t wresult = write(pd, &c, 1);
  return wresult == 1 ? 1 : 0;
}

int putchs(char *str, int pd)
{
  char c;
  unsigned n = 0;
  while ((c = *str++) != '\0')
  {
    n += putch(c, pd);
  }
  return n;
}
