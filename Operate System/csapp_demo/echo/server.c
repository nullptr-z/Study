#include <csapp.h>

void echo(int connfd);
int main(int argc, char **argv)
{
  int listenfd, connfd;
  socklen_t clientlen;
  // sockaddr_storage 比 sockaddr_in 有更好的兼容性，可以存储任何类型的套接字地址
  struct sockaddr_storage clientaddr;
  char client_hostname[MAXLINE], client_port[MAXLINE];

  if (argc != 2)
  {
    fprintf("出错了:%s", argv[0]);
    exit(0);
  }
  listenfd = Open_listenfd(1887);

  while (1 && listenfd > 0)
  {
    clientlen = sizeof(clientaddr);
    connfd = Accept(listenfd, (SA *)&clientaddr, &clientlen);
    // Rio_readlineb();
    getnameinfo((SA *)&clientaddr, clientlen, client_hostname, MAXLINE, client_port, MAXLINE, 0);
    printf("Connected to (%s:%s)\n", client_hostname, client_port);
    echo(connfd);
    Close(connfd);
  }
}

void echo(int connfd)
{
  char buf[MAXLINE];
  rio_t rio;
  size_t n;

  rio_readinitb(&rio, connfd);

  while ((n = Rio_readlineb(&rio, buf, MAXLINE)) != 0)
  {
    printf("client send size: %d, data: %s", n, buf);

    // scanf("%s", buf);

    Rio_writen(connfd, "已收到\n", 20);
  };
}
