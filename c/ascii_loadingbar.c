#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main(void) {
    int i, j;
    putchar('\n');
    for(i=0 ; i<=100 ; i++) {
	//Begin Esc
	printf("\033[F(%.3d%%) [", i);
	for(j=0 ; j<i/2 ; j++)
	    putchar('0');
	for(    ; j<100/2 ; j++)
	    putchar('_');
	putchar(']');
	putchar('\n');
	usleep(40000);
    }
    exit(EXIT_SUCCESS);
}
