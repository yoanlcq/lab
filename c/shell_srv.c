#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <signal.h>
#include <unistd.h>

#define SH_BUFFER_SIZE 4096

#define CMD_EXIT -1

int cmd_exit(int argc, char *argv[], int sockfd, struct sockaddr *addr, socklen_t len) {
    if(strcmp(argv[0], "bye")==0)
        sendto(sockfd, "Goodbye.\n", 10, 0, addr, len);
    return CMD_EXIT;
}

struct cmd {
    const char name[32];
    int (*func)(int argc, char *argv[], int sockfd, struct sockaddr *addr, socklen_t len);
};

#define CMD_NUM_COMMANDS 3
const struct cmd _commands[CMD_NUM_COMMANDS] = {
    {"exit", &cmd_exit},
    {"quit", &cmd_exit},
    {"bye",  &cmd_exit}
};

void _shLoop(struct sockaddr *addr, socklen_t len) {
    int bytes_read;
    char buffer[SH_BUFFER_SIZE];
    char prompt[SH_BUFFER_SIZE];
    char *token;
    char **argv;
    int sockfd;
    int cmd_res;
    int argc = 0;
    unsigned i, argv_mallocsize;
    
    sockfd = socket(AF_INET6, SOCK_DGRAM, 0);
    
    sendto(sockfd, "Welcome to the FATE shell !\n", 28, 0, addr, len);
    strcpy(prompt, "yoon@Sailor ~/\n> ");

    for(;;) {
        sendto(sockfd, prompt, strlen(prompt), 0, addr, len);
        sendto(sockfd, "", 1, 0, addr, len);

        bytes_read = recvfrom(sockfd, buffer, SH_BUFFER_SIZE, 0, addr, &len);
        token = strtok(buffer, " \t\n");
        if(!token)
            continue;
        for(i=0 ; i<CMD_NUM_COMMANDS ; ++i) {
            if(strcmp(token, _commands[i].name) != 0) 
                continue;

            argv_mallocsize = 16;
            argv = malloc(argv_mallocsize*sizeof(char*));
            argv[0] = malloc((strlen(token)+1)*sizeof(char));
            strcpy(argv[0], token);
            for(argc=1;;) {
                token = strtok(NULL, " \t\n");
                if(!token)
                    break;
                ++argc;
                if(argc > argv_mallocsize) {
                    argv_mallocsize <<= 1;
                    argv = realloc(argv, argv_mallocsize);
                }
                argv[argc-1] = malloc((strlen(token)+1)*sizeof(char));
                strcpy(argv[argc-1], token);
            }

            cmd_res = _commands[i].func(argc, argv, sockfd, addr, len);

            for(; argc != 0 ;) {
                --argc;
                free(argv[argc]);
            }
            free(argv);
            break;
        }
        if(cmd_res == CMD_EXIT)
            break;
        sendto(sockfd, "I don't understand this command.\n", 34, 0, addr, len);
    }
    sendto(sockfd, "\0", 2, 0, addr, len);
    close(sockfd);
}

void chldhandler(int s) {
    while(waitpid(-1, NULL, WNOHANG) > 0);
}

void server(void) {
    int bytes_read;
    char buffer[SH_BUFFER_SIZE];
    int sockfd;
    struct sockaddr_in6 server_addr, client_addr;
    struct sigaction sa;

    memset(&sa, 0, sizeof(struct sigaction));
    sa.sa_flags = SA_RESTART;
    sa.sa_handler = &chldhandler;

    sigaction(SIGCHLD, &sa, NULL);


    socklen_t socklen = sizeof(struct sockaddr_in6);
    
    sockfd = socket(AF_INET6, SOCK_DGRAM, 0);
    
    memset(&server_addr, 0, socklen);
    server_addr.sin6_family = AF_INET6;
    server_addr.sin6_addr = in6addr_any;
    server_addr.sin6_port = htons(55555);
    bind(sockfd, (const struct sockaddr*) &server_addr, socklen);

    
    for(;;) {
        bytes_read = recvfrom(sockfd, buffer, SH_BUFFER_SIZE, 0, (struct sockaddr*) &client_addr, &socklen);
        if(bytes_read > 0) {
            switch(fork()) {
                case -1: break;
                case 0: 
                    close(sockfd);
                    _shLoop((struct sockaddr*) &client_addr, socklen); return;
                default: break;
            }
        }
    }
}


int main(int argc, char *argv[]) {

    server();

    exit(EXIT_SUCCESS);
}
