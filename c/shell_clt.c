#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>

#define SH_BUFFER_SIZE 4096

void client(void) {
    char buffer[SH_BUFFER_SIZE];
    int bytes_read;
    int sockfd;
    struct sockaddr_in6 server_addr;
    socklen_t socklen = sizeof(struct sockaddr_in6);

    sockfd = socket(AF_INET6, SOCK_DGRAM, 0);

    memset(&server_addr, 0, socklen);
    server_addr.sin6_family = AF_INET6;
    server_addr.sin6_addr = in6addr_loopback;
    server_addr.sin6_port = htons(55555);
 
    sendto(sockfd, "Hello !\n", 9, 0, (const struct sockaddr*) &server_addr, socklen);
    for(;;) {
        for(;;) {
            bytes_read = recvfrom(sockfd, buffer, SH_BUFFER_SIZE, 0, (struct sockaddr*)&server_addr, &socklen);
            if(bytes_read <= 0 || (bytes_read<=2 && buffer[0]=='\0'))
                break;
            write(1, buffer, bytes_read);
        }
        if(bytes_read==2 && buffer[0]+buffer[1]<=0)
            break;
        bytes_read = read(0, buffer, SH_BUFFER_SIZE);
        sendto(sockfd, buffer, bytes_read, 0, (const struct sockaddr*) &server_addr, socklen);
    }
    close(sockfd);
}

int main(int argc, char *argv[]) {

    client();

    exit(EXIT_SUCCESS);
}
