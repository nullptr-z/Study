
#include <netinet/in.h>
#include <unistd.h>
#include <string.h>
#include <stdio.h>

int main()
{
  struct sockaddr_in sad;
  struct sockaddr_in cad;
  int clen = sizeof(cad);

  int socket_fd = socket(PF_INET, SOCK_DGRAM, 0);

  memset((char *)&sad, 0, sizeof(sad)); // 初始化；清除内存中的垃圾
  sad.sin_family = AF_INET;
  sad.sin_addr.s_addr = INADDR_ANY; // 本机IP地址
  sad.sin_port = 888;               // htons((u_short)port);

  int is_bind = bind(socket_fd, (struct sockaddr *)&sad, sizeof(sad));

  char buf[128];
  char *send_msg = "已收到";

  if (!is_bind)
  {
    int rev_result = recvfrom(socket_fd, buf, sizeof(buf), 0, (struct sockaddr *)&cad, &clen);

    int resp = sendto(socket_fd, send_msg, strlen(send_msg) + 1, 0, (struct sockaddr *)&cad, sizeof(cad));
    printf("server buf: %s  \n", buf);
    printf("resp is success: %d \n", resp);
  }
}
