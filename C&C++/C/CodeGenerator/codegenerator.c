#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>
#include <ctype.h>

int main(int argc, char const *argv[])
{
    if (argc == 2)
    {
        const char *fileName = argv[1];
        char fileNames[16];
        for (size_t d; fileName[d] != '\0'; d++)
            fileNames[d] = fileName[d];
        strcat(fileNames, ".js");
        int index_fd, index_template_fd;
        if ((index_fd = creat(fileNames, 0666)) != -1)
            if ((index_fd = open(fileNames, O_WRONLY, 0)) != -1)
                if ((index_template_fd = open("index_template", O_RDONLY, 0)) != -1)
                {
                    char temp[BUFSIZ], copy[BUFSIZ + 100], c;
                    int read_count;
                    while ((read_count = read(index_template_fd, temp, BUFSIZ)) > 0)
                    {
                        for (unsigned i = 0, j = 0; i < read_count; i++, j++)
                        {
                            c = temp[i];
                            if (c == '$')
                                switch (temp[++i])
                                {
                                case '1':
                                    for (size_t x = 0; fileName[x] != '\0'; x++)
                                    {
                                        copy[j++] = fileName[x];
                                        ++read_count;
                                    }
                                    read_count -= 2;
                                    --j;
                                    continue;
                                case '2':
                                    for (size_t x = 0; fileName[x] != '\0'; x++)
                                    {
                                        copy[j++] = tolower(fileName[x]);
                                        ++read_count;
                                    }
                                    read_count -= 2;
                                    --j;
                                    continue;
                                default:
                                    break;
                                }
                            copy[j] = c;
                        }

                        if (write(index_fd, copy, read_count) != read_count)
                            perror("拷贝模板时遇到异常被中断");
                    }
                }
                else
                    perror("index模板打开失败");
            else
                perror("文件打开失败");
        else
            perror("创建文件失败");
    }
    else
        perror("文件参数数量不对");
    return 0;
}
