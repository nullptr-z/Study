#include "stdio.c"
#include "fopen.c"
 FILE _iob[OPEN_MAX] = { /* stdin, stdout, stderr */
 { 0, (char *) 0, (char *) 0, _READ, 0 },
 { 0, (char *) 0, (char *) 0, _WRITE, 1 },
 { 0, (char *) 0, (char *) 0, _WRITE | _UNBUF, 2 }
 };
int main(int argc, char const *argv[])
{
	FILE* fd;
	fd=fopens(argv[1],"r");
	char c[10];
	for (size_t i = 0; (c[i] = getc(fd) )!= EOF; i++);
	write(1,c,10);


	return 0;
}
