
#include <netinet/in.h>
#include <unistd.h>
#include <string.h>
#include <netdb.h>
#include <stdio.h>

int main()
{
  struct sockaddr_in sad;
  struct sockaddr_in svad;
  int svlen = sizeof(svad);

  /** 本地socket创建 */
  int socket_fd = socket(PF_INET, SOCK_DGRAM, 0);
  struct hostent *ptrh = gethostbyname("127.0.0.1");
  memset((char *)&sad, 0, sizeof(sad));
  memcpy(&sad.sin_addr, ptrh->h_addr, ptrh->h_length);
  sad.sin_family = AF_INET;
  sad.sin_port = 888; // htons((u_short)port);

  // int is_bind = bind(socket_fd, (struct sockaddr *)&sad, sizeof(sad));

  char buf[128];
  char *send_msg = "你收到了吗";

  int sen_resut = sendto(socket_fd, send_msg, strlen(send_msg) + 1, 0, (struct sockaddr *)&sad, sizeof(sad));

  int resp = recv(socket_fd, buf, sizeof(buf), 0);
  printf("resp buf: %s", buf);
}
