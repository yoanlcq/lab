#include <stdlib.h>
#include <stdio.h>
#include <setjmp.h>

/* internal */

jmp_buf obuf, abuf;

void alert() {
    //Save state
    int state = setjmp(obuf);
    if(state==0 || state==2) {
	//Jump to onAlert()
	longjmp(abuf, 1)
    } else {
	//Restore state
	longjmp(buf, 1);
    }
}



/* external */

void onAlert() {
    printf("ALERTE !\n");
    longjmp(buf, 2); //Jump back
}

void *friend(void *arg) {
    for(;;) {
	sleep(2);
	alert();
    }
}

int main(void) {
    pthread_t thread;
    pthread_create(&thread, NULL, &friend, NULL);
    for(;;) {
	printf("Rien à signaler.\n");
	sleep(1);
    }
    exit(EXIT_SUCCESS);
}
