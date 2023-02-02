#include <stdio.h>
#include "minprintf.c"
#include "minscanf.c"

int main()
{
	int len = 10;
	printf("%x\n", len);

	char *str = "print";
	printf("%-5.*s:%.*s\n", 3, str, 4, str);

	minprintf("%d  %s\n", len, str);

	char *str1 = "  sscanf 33s3";
	sscanf(str1, "%s%d", &str, &len);
	printf("%s	%d\n", &str, len);

	char *str3;
	int len1 = 11;
	minscanf("%s%d", &str3, &len1);
	printf("minscanf: %s\n", &str3);
	printf("minscanf:  %d\n", len1);

	return 0;
}