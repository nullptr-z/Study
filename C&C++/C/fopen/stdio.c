#include <stdlib.h>
#include <unistd.h>

#define NULL 0
#define EOF (-1)
#define BUFSIZ 1024
#define OPEN_MAX 20 /* max #files open at once */
typedef struct _iobuf
{
	int cnt;		/* 剩余字符 */
	char *ptr;	/*下一个字符位置*/
	char *base; /*缓冲区的位置*/
	int flag;		/*文件访问方式*/
	int fd;			/*文件描述符*/
} FILE;
extern FILE _iob[OPEN_MAX];
#define stdin (&_iob[0])
#define stdout (&_iob[1])
#define stderr (&_iob[2])
enum _flags
{
	_READ = 01,	 /*打开文件以供读取*/
	_WRITE = 02, /*文件打开以进行写入*/
	_UNBUF = 04, /*文件未缓冲*/
	_EOF = 010,	 /*此文件已发生EOF */
	_ERR = 020	 /*此文件上发生错误*/
};
int _fillbuf(FILE *);
int _flushbuf(int, FILE *);
#define feof(p) ((p)->flag & _EOF) != 0)
#define ferror(p) ((p)->flag & _ERR) != 0)
#define fileno(p) ((p)->fd)
#define getc(p) (--(p)->cnt >= 0                  \
										 ? (unsigned char)*(p)->ptr++ \
										 : _fillbuf(p))
// #define putc(x, p) (--(p)->cnt >= 0         \
// 												? *(p)->ptr++ = (x) \
// 												: _flushbuf((x), p))
#define getchar() getc(stdin)
#define putcher(x) putc((x), stdout)


int _fillbuf(FILE *fp)
{
	int bufsize;
	if ((fp->flag & (_READ | _ERR)) != _READ)
		return EOF;
	bufsize = (fp->flag & _UNBUF) ? 1 : BUFSIZ;
	if (fp->base == NULL) /* no buffer yet */
		if ((fp->base = (char *)malloc(bufsize)) == NULL)
			return EOF; /* can't get buffer */
	fp->ptr = fp->base;
	fp->cnt = read(fp->fd, fp->ptr, bufsize);
	if (--fp->cnt < 0)
	{
		if (fp->cnt == -1)
			fp->flag |= _EOF;
		else
			fp->flag |= _ERR;
		fp->cnt = 0;
		return EOF;
	}
	return (unsigned char)*fp->ptr++;
}
