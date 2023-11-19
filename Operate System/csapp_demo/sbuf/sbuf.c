#include "csapp.h"
#include "sbuf.h"

void sbuf_init(sbuf_t *sf, int n)
{
  sf->buf = Malloc(n * sizeof(int));
  sf->n = n;
  sf->front = sf->rear = 0;
  sf->mutex = sem_open("mutex", O_CREAT | O_EXCL, S_IRWXU, 1);
  sf->slots = sem_open("slots", O_CREAT | O_EXCL, S_IRWXU, n);
  sf->items = sem_open("items", O_CREAT | O_EXCL, S_IRWXU, 0);
}

void sbuf_deinit(sbuf_t *sf)
{
  Free(sf->buf);

  sem_close(sf.mutex);
  sem_close(sf->slots);
  sem_close(sf->items);
}

void sbuf_insert(sbuf_t *sf, int item)
{
  P(&sf->slots);
  P(&sf->mutex);
  sf->buf[++sf->rear % sf->n] = item;
  V(&sf->mutex);
  V(&sf->items);
}

void sbuf_remove(sbuf_t *sf)
{
  int item;
  P(&sf->items);
  P(&sf->mutex);
  item = sf->buf[++sf->front % sf->n];
  V(&sf->mutex);
  V(&sf->slots);
  return item;
}
