#include "csapp.h"

void doit(int);

typedef struct sockaddr SA;

int main()
{
  int listen_fd, connect_fd, optval = 1;
  struct sockaddr_in sad, cad;
  socklen_t client_ad_len = sizeof(cad);
  char hostname[MAXLINE], port[MAXLINE];
  // struct addrinfo ad_info, *list_ad_info;

  // getaddrinfo("zheng-air.local", NULL, &ad_info, &list_ad_info);

  bzero((char *)&sad, sizeof(sad));
  sad.sin_family = AF_INET;
  sad.sin_port = htons((u_short)1999);
  // sad.sin_addr = ((struct sockaddr_in *)list_ad_info->ai_addr)->sin_addr;

  listen_fd = socket(AF_INET, SOCK_STREAM, 0);
  setsockopt(listen_fd, SOL_SOCKET, SO_REUSEADDR, (const void *)&optval, sizeof(int)); // 消除端口号被占用的错误
  int bind_rt = bind(listen_fd, (SA *)&sad, sizeof sad);

  int listen_rt = listen(listen_fd, 1024);

  while (1)
  {
    connect_fd = accept(listen_fd, (SA *)&cad, &client_ad_len);
    getnameinfo((SA *)&cad, client_ad_len, hostname, MAXLINE, port, MAXLINE, 0);
    printf("Accept connect from (%s, %d)\n", hostname, ntohl(sad.sin_port));

    doit(connect_fd);
    close(connect_fd);
  }
}
