#include <unistd.h>
#include <stdio.h>

int main(){
  char ch;
  scanf("%c", &ch);
  printf("%d\n", getgid());

  return 0;
}
