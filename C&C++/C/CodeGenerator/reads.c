
#include <stdio.h>
#include <unistd.h> 
#include <sys/fcntl.h>

int main(int argc, char const *argv[])
{

    char str[BUFSIZ];
    int readCount;
    while ((readCount = read(0, str, BUFSIZ)) > 0)
    {
        printf("str: %s\n", str);
        if (write(1, str, readCount) != BUFSIZ)
        {
            // write(0, str, 5);
        }
        putchar('\n');
    }

    return 0;
}
