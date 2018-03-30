#include<stdio.h>int main(void){char c;FILE*f=fopen(__FILE__,"r");while((c=fgetc(f))!=EOF)putchar(c);fclose(f);return 0;}
