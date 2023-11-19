#include <csapp.h>

int main(int argc, char **argv)
{
  struct addrinfo *p, *listp, hints;
  char buf[MAXLINE];
  int rc, flags;
  char *opt = argv[1];

  if (argc != 2)
  {
    fprintf(stderr, "usage：%s <domain name>\n", argv[0]);
    exit(0);
  }

  /** get a list of addinfo records记录*/
  memset(&hints, 0, sizeof(struct addrinfo));
  hints.ai_family = AF_INET;
  hints.ai_socktype = SOCK_STREAM;
  if ((rc = getaddrinfo("twitter.com", NULL, &hints, &listp)) != 0)
  {
    fprintf(stderr, "getaddrinfo error: %s\n", gai_strerror(rc));
    exit(1);
  }

  /** walk`遍历 the list and display each`每个 IP addrinfo */
  // display address string instead of domain name
  flags = NI_NUMERICHOST;
  for (p = listp; p; p = p->ai_next)
  {
    getnameinfo(p->ai_addr, p->ai_addrlen, buf, MAXLINE, NULL, 0, flags);
    printf("%s\n", buf);
  }

  freeaddrinfo(listp);

  exit(0);
}
