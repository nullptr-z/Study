#include "csapp.h"

typedef struct
{
  int *buf;    // 缓冲区指针
  int n;       // 缓冲区长度
  int front;   // 缓冲器首指针
  int rear;    // 缓冲器尾指针
  sem_t mutex; // buf 互斥访问
  sem_t slots; // 可用槽位
  sem_t items; // 可用项目
} sbuf_t;

void sbuf_init(sbuf_t *sf, int n);
// 释放
void sbuf_deinit(sbuf_t *sf);
void sbuf_insert(sbuf_t *sf, int item);
void sbuf_remove(sbuf_t *sf);
