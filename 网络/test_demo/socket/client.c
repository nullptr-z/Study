#include <netinet/in.h>
#include <stdlib.h>
#include <string.h>
#include <netdb.h>
#include <unistd.h>

int main(char argc, char *argv[])
{
  // 服务器主机信息
  struct sockaddr_in sad;
  // 服务器 host
  struct hostent *ptrh;
  int client_socket;

  char *sentence = "Im Client";
  char modifiedSentence[128];

  // char *host = argv[1];
  // int port = atoi(argv[2]); // 将端口号转为整数类型
  char *host = "127.0.0.1";
  int port = 889;

  // PF_INET: intelnet地址簇
  // SOCK_STREAM：类型为 TCP 的 socket
  // 0：自动选择一个端口
  // 返回创建的的套接字文件描述符
  client_socket = socket(PF_INET, SOCK_STREAM, 0);
  // 清除sad，未初始化的结构内存中可能包含很多未知数据
  memset((char *)&sad, 0, sizeof(sad));

#pragma region 开始写入sad
  // intelnet地址簇
  sad.sin_family = AF_INET;
  // sad.sin_port = htons((u_short)port);              // 将端口转为大端字序，这是网络传输规范
  sad.sin_port = port;
  ptrh = gethostbyname(host);                          // 返回主机的信息
  memcpy(&sad.sin_addr, ptrh->h_addr, ptrh->h_length); // IP地址拷贝到sad.sin_addr
  int is_connect = connect(client_socket, (struct sockaddr *)&sad, sizeof(sad));

  if (!is_connect)
  {
    int write_n = write(client_socket, sentence, strlen(sentence) + 1);

    int read_n = read(client_socket, modifiedSentence, sizeof(modifiedSentence));
    printf("read_n: %d,%s \n", read_n, modifiedSentence);
  }
#pragma endregion
}
