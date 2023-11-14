#include <stdio.h>

int main(int argc, char const *argv[])
{
    void compareds(FILE * original);

    FILE *original;
    char *arg1, *arg2;
    if (argc < 2)
    {
        fprintf(stdout, "参数错误,请传入需要读取的文件!");
    }
    else
    {
        original = fopen(argv[1], "r");
        compareds(original);
        fclose(original);
    }
    return 0;
}

void compareds(FILE *original)
{
    int o;
    while ((o = getc(original)) != EOF)
    {
        if (o != '\n')
            putchar(o);
        else
            break;
    }
    putchar('\n');

    int line = 10;
    char str[line][10];
    for (size_t i = 0; i < line; i++)
    {
        if (fgets(str[i], 10, original) != NULL)
            printf("str: %s\n", str[i]);
        else
            break;
    }
}