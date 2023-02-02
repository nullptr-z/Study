#include "csapp.h"

#define N 2

char **ptr;
void *thread(void *argv);

int main()
{
  pthread_t pid;
  char *msg[N] = {
      "Hello",
      "hello",
  };

  ptr = msg;
  for (size_t i = 0; i < N; i++)
  {
    pthread_create(&pid, NULL, thread, (void *)i);
  }
  pthread_exit(NULL);
}

void *thread(void *argv)
{
  size_t my_id = (size_t)argv;
  static int cnt = 0;
  if (my_id == 0)
  {
    sleep(1);
  }

  printf("[%zu]: %s, cnt: %d\n", my_id, ptr[my_id], ++cnt);
  return NULL;
}
