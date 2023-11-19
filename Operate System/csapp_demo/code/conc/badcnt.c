/*
 * badcnt.c - An improperly synchronized counter program
 */
/* $begin badcnt */
#include "csapp.h"

void *thread(void *vargp); /* Thread routine prototype */

/* Global shared variable */
volatile int cnt = 0; /* Counter */

struct thread_struct
{
    sem_t *mutex;
    int *niters;
};

int main(int argc, char **argv)
{
    int niters;
    pthread_t tid1, tid2;
    struct thread_struct ts;
    // sem_init(&mutex, 0, 1);
    sem_t *mutex = sem_open(argv[1], O_CREAT | O_EXCL, S_IRWXU, 1);
    /* Check input argument */
    // if (argc != 2)
    // {
    //     printf("usage: %s <niters>\n", argv[0]);
    //     exit(0);
    // }
    // niters = atoi(argv[1]);
    niters = atoi("10000");

    ts.niters = &niters;
    ts.mutex = mutex;
    /* Create threads and wait for them to finish */
    Pthread_create(&tid1, NULL, thread, (void *)&ts);
    Pthread_create(&tid2, NULL, thread, (void *)&ts);
    Pthread_join(tid1, NULL);
    Pthread_join(tid2, NULL);

    /* Check result */
    if (cnt != (2 * niters))
        printf("BOOM! cnt=%d\n", cnt);
    else
        printf("OK cnt=%d\n", cnt);
    exit(0);
}

/* Thread routine */
void *thread(void *vargp)
{
    struct thread_struct ts = *(struct thread_struct *)vargp;
    int i, niters = *ts.niters;

    for (i = 0; i < niters; i++) // line:conc:badcnt:beginloop
    {
        P(ts.mutex);
        cnt++; // line:conc:badcnt:endloop
        V(ts.mutex);
    }

    return NULL;
}
/* $end badcnt */
