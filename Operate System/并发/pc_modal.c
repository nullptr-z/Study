#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

#define BUFFER_SIZE 5

struct SharedData
{
  int buffer[BUFFER_SIZE];
  int count;
  int in;
  int out;
  pthread_mutex_t mutex;
  pthread_cond_t full;
  pthread_cond_t empty;
};

void initialize(struct SharedData *shared)
{
  shared->count = 0;
  shared->in = 0;
  shared->out = 0;
  pthread_mutex_init(&shared->mutex, NULL);
  pthread_cond_init(&shared->full, NULL);
  pthread_cond_init(&shared->empty, NULL);
}

void destroy(struct SharedData *shared)
{
  pthread_mutex_destroy(&shared->mutex);
  pthread_cond_destroy(&shared->full);
  pthread_cond_destroy(&shared->empty);
}

void produce(struct SharedData *shared, int item)
{
  shared->buffer[shared->in] = item;
  shared->in = (shared->in + 1) % BUFFER_SIZE;
  shared->count++;
}

int consume(struct SharedData *shared)
{
  int item = shared->buffer[shared->out];
  shared->out = (shared->out + 1) % BUFFER_SIZE;
  shared->count--;
  return item;
}

void *producer(void *arg)
{
  struct SharedData *shared = (struct SharedData *)arg;

  for (int i = 0; i < 10; i++)
  {
    pthread_mutex_lock(&shared->mutex);

    while (shared->count == BUFFER_SIZE)
    {
      pthread_cond_wait(&shared->empty, &shared->mutex);
    }

    int item = rand() % 100;
    produce(shared, item);
    printf("Produced: %d\n", item);

    pthread_cond_signal(&shared->full);
    pthread_mutex_unlock(&shared->mutex);
  }

  pthread_exit(NULL);
}

void *consumer(void *arg)
{
  struct SharedData *shared = (struct SharedData *)arg;

  for (int i = 0; i < 10; i++)
  {
    pthread_mutex_lock(&shared->mutex);

    while (shared->count == 0)
    {
      pthread_cond_wait(&shared->full, &shared->mutex);
    }

    int item = consume(shared);
    printf("Consumed: %d\n", item);

    pthread_cond_signal(&shared->empty);
    pthread_mutex_unlock(&shared->mutex);
  }

  pthread_exit(NULL);
}

int main()
{
  pthread_t producer_thread, consumer_thread;
  struct SharedData shared;

  // 初始化共享资源和互斥锁
  initialize(&shared);

  // 创建生产者和消费者线程
  pthread_create(&producer_thread, NULL, producer, &shared);
  pthread_create(&consumer_thread, NULL, consumer, &shared);

  // 等待线程结束
  pthread_join(producer_thread, NULL);
  pthread_join(consumer_thread, NULL);

  // 销毁互斥锁和条件变量
  destroy(&shared);

  return 0;
}
