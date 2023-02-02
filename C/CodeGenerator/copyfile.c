#include <stdio.h>

int main(int argc, char *argv[])
{
	FILE *fp,*fp1;
	void filecopy(FILE *, FILE *);

	if (argc == 1)
	{
		filecopy(stdin, stdout);
	}
	else
		while (--argc > 0)
			if ((fp = fopen(*++argv, "r")) == NULL)
			{	// 打开文件失败，例如文件不存在或没有权限
				fprintf(stderr, "无法打文件：%s, 参数（%d）\n", *argv, argc);
				return 1;
			}
			else
			{
				// fp1 = fopen(*++argv, "w");
				// --argc;
				// filecopy(fp, fp1);
				filecopy(fp, stdout);
				fclose(fp);
			}
	// feof(FILE *) 文件达到结尾时返回非0
	if (ferror(stdout)) // 输出错误（例如磁盘满了），ferror抛出非0值
	{
		fprintf(stderr, "%s: error writing stdout\n", argv[0]);
		exit(2);
	}
	return 0;
}

void filecopy(FILE *infp, FILE *outfp)
{
	int c;
	while ((c = getc(infp) )!= EOF)
	{
		putc(c, outfp);
	}
}
