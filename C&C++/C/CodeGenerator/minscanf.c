#include <stdio.h>
#include <stdarg.h>

void minscanf(char *fomat, ...)
{
	va_list ap;
	char *p, *sval;
	int *ival;
	double *dval;

	va_start(ap, fomat);
	for (p = fomat; *p; p++)
	{
		if (*p == '%')
		{
			char c = getchar();
			switch (*++p)
			{
			case 'd':
				ival = va_arg(ap, int *);
				*ival = 0;
				while (c >= '0' && c <= '9')
				{
					if (*ival > 0)
					{
						*ival *= 10;
					}
					*ival += (int)c - 48;
					c = getchar();
				}
				break;


			case 's':
				sval = va_arg(ap, char *);
				for (int i = 0; (c != '\t' && c != '\b' &&c != '\n' && c != '\0'&& c != '\40'); i++)
				{
					*(sval+i) = c;
					c = getchar();
				}
				break;

			default:
				break;
			}
		}
	}
}