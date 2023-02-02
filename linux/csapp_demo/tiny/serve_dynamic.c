#include "csapp.h"

void serve_dynamic(int fd, char *filename, char *cgiargs)
{
  char buf[MAXLINE], *emptylist[] = {NULL};

  /// 返回第一部分HTTP响应
  sprintf(buf, "HTTP/1.0 200 OK\r\n");
  Rio_writen(fd, buf, strlen(buf));
  sprintf(buf, "Server: Tiny Web Server\r\n");
  Rio_writen(fd, buf, strlen(buf));

  if (fork() == 0) // child
  {
    /// 真实服务器会把 CGI 变量都设置在这里
    setenv("QUERY_STRING", cgiargs, 1);
    Dup2(fd, STDOUT_FILENO);              // 将stdout重定向到客户端,cgi通过stdout文件描述符向客户端 socket 写入数据
    Execve(filename, emptylist, environ); // 运行 CGI 程序
  }

  Wait(NULL); // 等待子程序
}
