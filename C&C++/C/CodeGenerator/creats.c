#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/file.h>

int main(int argc, char *argv[])
{
	char file[8];
	int fd;
	fd = open(argv[1], O_RDONLY, 0);
	if (fd == -1)
	{
		int ct = creat(argv[1], 0666);
		if (ct != -1)
		{
			printf("已创建文件：%s\n", argv[1]);
		}
		else
		{
			printf("无法打开或建文件：%s\n", argv[1]);
		}
	}
	else
	{
		int cc = 0;
		if ((cc = write(fd, "恭喜你被创建了", 21)) == 21)
		{
			close(fd);
			fd = open(argv[1], O_RDWR, 0);
			if (read(fd, file, sizeof(file)) > 0)
			{
				printf("成功打开文件：%s ,并写入内容：%s", argv[1], file);
				write(1, file, sizeof(file));
			}
		}
		else
		{
			printf("写入文件失败: %d", cc);
		}
	}
	return 0;
}
