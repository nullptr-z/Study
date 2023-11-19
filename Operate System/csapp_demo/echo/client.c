#include <netdb.h>
#include <csapp.h>

int Open_clientfd(char *hostname, int port);
int main(int argc, char **argv)
{
  int clientfd, port;
  char *host, buf[MAXLINE];
  rio_t rio;

  if (argc != 2)
  {
    fprintf(stderr, "usage`用法: %s <host> <port>\n", argv[0]);
  }

  host = argv[1];
  port = 1887;

  clientfd = Open_clientfd(host, port);
  rio_readinitb(&rio, clientfd);

  while (Fgets(buf, MAXLINE, stdin) != NULL)
  {
    Rio_writen(clientfd, buf, sizeof(buf));
    Rio_readlineb(&rio, buf, MAXLINE);
    Fputs(buf, stdout);
  }

  Close(clientfd);
}
