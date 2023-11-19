// #include <csapp.h>
#include "csapp.h"
#include "dirent.h"

int main(int argc, char **argv)
{
  DIR *streamp;
  struct dirent *dep;
  int n;

  streamp = opendir("/Users/zhouzheng/");

  while ((dep = readdir(streamp)) != NULL)
  {
    printf("%s\t", dep->d_name);
  }

  closedir(streamp);
}
