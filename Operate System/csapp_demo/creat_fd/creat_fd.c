#include <arpa/inet.h>
#include <stdlib.h>
#include <netdb.h>
#include <string.h>
#include <unistd.h>
#include <stdio.h>

#define MAXBUf 1024

typedef struct sockaddr SA;

int creat_server_fd(char *port)
{
  struct sockaddr_in server_ad, client_ad;
  socklen_t client_ad_len;
  // struct addrinfo hints, *listp, *p;
  int server_fd, connct_fd;
  char buf[MAXBUf];

  bzero((char *)&server_ad, sizeof(server_ad));
  server_ad.sin_family = AF_INET;
  server_ad.sin_port = htons((u_short)(atoi(port)));
  server_fd = socket(AF_INET, SOCK_STREAM, 0);
  int binds = bind(server_fd, (SA *)&server_ad, sizeof(server_ad));

  listen(server_fd, 1024);

  while (1)
  {
    client_ad_len = sizeof(client_ad);
    connct_fd = accept(server_fd, (SA *)&client_ad, &client_ad_len);
    return connct_fd;
    // int n = read(connct_fd, buf, MAXBUf);
    // printf("c%s", buf);
  }
}

int creat_client_fd(char *hostname, char *port)
{
  struct sockaddr_in server_ad, *sockp;
  struct addrinfo hints, *listp;

  int client_fd;
  char buf[MAXBUf];

  // struct in_addr inaddr;
  // inaddr.s_addr = 146581696;
  // inet_ntop(AF_INET, &inaddr, buf, MAXBUf);
  // fputs(buf, stderr);

  bzero((char *)&hints, sizeof(hints));
  hints.ai_family = AF_INET;
  hints.ai_socktype = SOCK_STREAM;
  int get_in = getaddrinfo(hostname, NULL, &hints, &listp);

  bzero((char *)&server_ad, sizeof(server_ad));
  sockp = (struct sockaddr_in *)listp->ai_addr;

  server_ad.sin_family = AF_INET;
  server_ad.sin_port = htons((u_short)(atoi(port)));
  server_ad.sin_addr = sockp->sin_addr;
  client_fd = socket(AF_INET, SOCK_STREAM, 0);

  int connectfd = connect(client_fd, (SA *)&server_ad, sizeof(server_ad));
  return client_fd;
  // int w_c = write(client_fd, "hello", 20);
}

int main()
{
  // creat_server_fd("1887");
  creat_client_fd("zheng-air.local", "1888");
}
