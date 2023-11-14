#include <unistd.h>
#include "putch.c"
#include "getch.c"
#include "copy.c"
#include <fcntl.h>
#include <stdbool.h>

int dec(char c, const char *re)
{
  static int a = 0;
  if (c == re[a])
  {
    if (re[a + 1] == '\0')
    {
      a = 0;
      return aeq;
    }
    a++;
    return eq;
  }
  else if (a > 0)
  {
    a = 0;
    return neq;
  }
  return nn;
}

int main(int argc, char const *argv[])
{
  if (argc == 3)
  {
    const char *newFileName = argv[2];
    const char *templateFilename = argv[1];
    // const char *newFileName = "model";
    // const char *templateFilename = "models_template";

    int templatefd = open(templateFilename, O_RDONLY, 0);
    int newfd = creat(newFileName, 0666);
    char str[3];
    int count = 0;
    while ((count = getchs(str, 3, 0)) > 0)
    {
      printf("\nstr: %s", str);
      copy_rep(templatefd, newfd, dec, str, newFileName);
			off_t ft= lseek(newfd, 0, 0);

			templatefd = creat("newTemp", 0666);
			copy(newfd, templatefd);
			lseek(templatefd, 0, 0);
			lseek(newfd, 0, 0);
		}

    close(newfd);
    close(templatefd);
  }
  else
    putchs("cptu: 需要2个参数!", 1);

  return 0;
}
