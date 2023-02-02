#include <stdio.h>

int main(int argc, char const *argv[])
{
void compareds(FILE *original,FILE *compared);

    FILE *original, *compared;
    char *arg1, *arg2;
    if (argc < 2)
    {
        fprintf(stdout, "参数错误,需2个文件才能对比!");
    }
    else
    {
        original = fopen(argv[1], "r");
        compared = fopen(argv[2], "r");
        compareds(original, compared);
        fclose(original);
        fclose(compared);
    }
    return 0;
}

void compareds(FILE *original, FILE *compared)
{
    int o, c, i = 0;
    while ((o = getc(original)) != EOF || (c = getc(compared)) != EOF)
    {
        printf("c: %c\n", c);
        printf("o: %c    c: %c\n", o, c);
        if (o != c)
        {
            printf("i: %d\n", i);
            break;
        }
        i++;
    }
}
