#include <stdio.h>
#include <unistd.h>
#include <stdbool.h>
typedef int (*decFunc)(char ,const char*);

#define nn -1
#define neq 0
#define eq 1
#define aeq 2

int copy(int fdc, int pdp)
{
  char rs[BUFSIZ];
  int n;
  while ((n = read(fdc, rs, BUFSIZ)) > 0)
  {
    if (write(pdp, rs, n) != n)
      return -1;
  }
  return 1;
}

;

int copy_rep(int fdc, int pdp, decFunc dec,const char* re, const char *rep)
{
  char rs[BUFSIZ], copy[BUFSIZ + 100], c;
  int n,ii,jj;
  while ((n = read(fdc, rs, BUFSIZ)) > 0)
  {
    int dec_count = 0, j = 0;
    for (size_t i = 0 ; i < n; i++, j++)
    {
      switch (dec(rs[i], re))
      {
      case neq:
        dec_count = 0;
        break;
      case eq:
        ++dec_count;
        break;
      case aeq:
        j -= dec_count;
        dec_count = 0;
        for (size_t x = 0; (c = rep[x]) != '\0'; x++, j++)
          copy[j] = c;
        --j;
        continue;
      default:
        break;
      }
      copy[j] = rs[i];
    }
    if (write(pdp, copy, j) != j)
      return -1;
  }
  return 1;
}