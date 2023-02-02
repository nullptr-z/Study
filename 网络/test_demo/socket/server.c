#include <netinet/in.h>
#include <string.h>
#include <stdbool.h>
#include <unistd.h>
#include <stdio.h>

int main(char argc, char *argv[])
{
  struct sockaddr_in sad; // 本地主机信息
  struct sockaddr_in cad; // 客户端IP地址信息
  int clen = sizeof(cad);

  int welcome_socket, connection_socket;
  struct hostent *ptr;

  char clientSentence[128];
  char *capitalizedSentence = "Im Server";

  // int port = atoi(argv[1]);
  int port = 889;

  welcome_socket = socket(PF_INET, SOCK_STREAM, 0);
  memset((char *)&sad, 0, sizeof(sad));
  sad.sin_family = AF_INET;
  sad.sin_addr.s_addr = INADDR_ANY; // 本机IP地址
  sad.sin_port = port;              // htons((u_short)port);
  // 将 socket文件描述符，绑定到 socket
  int bind_result = bind(welcome_socket, (struct sockaddr *)&sad, sizeof(sad));

  listen(welcome_socket, 10); // 设置请求队列长度
  while (!bind_result)
  {
    // memset((char *)&cad, 0, sizeof(cad));
    connection_socket = accept(welcome_socket, (struct sockaddr *)&cad, &clen);
    printf("connection_socket: %d \n", connection_socket);
    // 从socket读取客户发送的数据
    int read_n = read(connection_socket, clientSentence, sizeof(clientSentence));
    printf("read_n: %d, %s \n", read_n, clientSentence);

    int write_n = write(connection_socket, capitalizedSentence, strlen(capitalizedSentence) + 1);
  }
  close(connection_socket);
}
