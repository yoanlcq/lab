/*
 * This game is better known as the "15-puzzle".
 * You should have no trouble compiling and running it on your system.
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <time.h>

#ifdef __WIN32
#define CLEAR() system("cls")
#else
#define CLEAR() printf("\033c")
#endif

typedef struct Taquin {
    unsigned width, height;
    unsigned curx, cury;
    unsigned ** tiles;
    unsigned moves, redoable_moves;
    unsigned char *moves_history;
} Taquin;

void Taquin_init(Taquin* t, unsigned w, unsigned h) {
    unsigned i, x, y;
    t->tiles = malloc(h*sizeof(unsigned*));
    for(i=0 ; i<h ; ++i)
        t->tiles[i] = malloc(w*sizeof(unsigned));
    for(i=y=0 ; y<h ; ++y)
        for(x=0 ; x<w ; ++x, ++i)
            t->tiles[y][x] = i;
    t->curx = t->cury = 0;
    t->width = w, t->height = h;
    t->moves = t->redoable_moves = 0;
    t->moves_history = malloc(1);
}

void Taquin_deinit(Taquin* t) {
    for(; t->height != 0 ;)
        free(t->tiles[--t->height]);
    free(t->tiles);
    free(t->moves_history);
}

void Taquin_swap(Taquin* t, unsigned sx, unsigned sy) {
    unsigned tmp;
    tmp = t->tiles[t->cury][t->curx];
    t->tiles[t->cury][t->curx] = t->tiles[sy][sx];
    t->tiles[sy][sx] = tmp;
}

bool Taquin_move(Taquin* t, unsigned char direction) {
    switch(direction) {
    case '<': 
        if(t->curx < t->width-1) {
            Taquin_swap(t, t->curx+1, t->cury);
            ++(t->curx);
            return true;
        }
        break;
    case '^': 
        if(t->cury < t->height-1) {
            Taquin_swap(t, t->curx, t->cury+1);
            ++(t->cury);
            return true;
        }
        break;
    case '>': 
        if(t->curx > 0) {
            Taquin_swap(t, t->curx-1, t->cury);
            --(t->curx);
            return true;
        }
        break;
    case 'v': 
        if(t->cury > 0) {
            Taquin_swap(t, t->curx, t->cury-1);
            --(t->cury);
            return true;
        }
        break;
    }
    return false;
}

void Taquin_print(const Taquin *t) {
    unsigned x, y;
    for(y=0 ; y<t->height ; ++y) {
        for(x=0 ; x<t->width ; ++x)
            if(!(x==t->curx && y==t->cury))
                printf(" %3u ", t->tiles[y][x]);
            else printf("     ");
        putchar('\n');
    }
    printf("(%u move%s)\n", t->moves, t->moves==1 ? "" : "s");
}

const char *TAQUIN_HOWTOPLAY = 
    "*** HOW TO PLAY ***\n"
    "\n"
    "This is a console 15-puzzle. The numbers you see are tiles that "
    "you can slide towards the empty slot.\n"
    "Your goal is to have the empty slot be on the top-left corner "
    "and the numbers arranged in crescent order when reading from "
    "left to right and top to bottom."
    "\n"
    "You interact with the puzzle by typing one or more command "
    "characters and pressing [Enter].\n"
    "List of command characters :\n"
    "< - Slide left\n"
    "^ - Slide up\n"
    "> - Slide right\n"
    "v - Slide down\n"
    "u - Undo last action\n"
    "r - Redo last undone action\n"
    "q - Quit\n"
    "h - Print this help\n";

void Taquin_exec(Taquin* t, unsigned char cmd) {
    switch(cmd) {
    case '<':
    case '^':
    case '>':
    case 'v':
        if(Taquin_move(t, cmd)) {
            ++(t->moves);
            t->moves_history = realloc(t->moves_history, t->moves);
            t->moves_history[t->moves-1] = cmd;
            t->redoable_moves = 0;
        }
        break;
    case 'u': 
        if(!t->moves)
            break;
        switch(t->moves_history[t->moves-1]) {
        case '<': cmd = '>'; break;
        case '^': cmd = 'v'; break;
        case '>': cmd = '<'; break;
        case 'v': cmd = '^'; break;
        }
        Taquin_move(t, cmd);
        --(t->moves); 
        ++(t->redoable_moves);
        break;
    case 'r': 
        if(!t->redoable_moves)
            break;
        --(t->redoable_moves);
        Taquin_move(t, t->moves_history[t->moves]);
        ++(t->moves);
        break;
    }
}


const unsigned char TAQUIN_INPUT[4] = {'^', '>', 'v', '<'};

void Taquin_shuffle(Taquin* t, unsigned moves) {
    unsigned char previous, current;
    for(previous = 1 ; moves ;) {
        current = rand()%4;
        if(current != (previous+2)%4) {
            if(Taquin_move(t, TAQUIN_INPUT[current])) {
                previous = current;
                moves--;
            }
        }
    }
}

bool Taquin_is_resolved(const Taquin *t) {
    unsigned i, x, y;
    for(i=y=0 ; y<t->height ; ++y)
        for(x=0 ; x<t->width ; ++x, ++i)
            if(t->tiles[y][x] != i)
                return false;
    return true;
}

int scan_int() {
    unsigned char buf[32];
    unsigned i;

    for(i=0 ; ; ) {
        buf[i] = getchar();
        if(buf[i]=='\n' || buf[i]==EOF)
            break;
        if(i<31)
            ++i;
    }
    buf[i] = '\0';

    return strtol(buf, NULL, 0);
}

int main(void) {
    Taquin t;
    unsigned w, h, s;
    unsigned char c;

    CLEAR();
    puts("Welcome to Taquin!\n");
    puts(TAQUIN_HOWTOPLAY);
    do {
        printf("Number of columns (between 2 and 255) : ");
        w = scan_int();
    } while(w<2 || w>255);
    do {
        printf("Number of rows (between 2 and 255) : ");
        h = scan_int();
    } while(h<2 && h>255);
    do {
        printf("Number of moves to perform for shuffling : ");
        s = scan_int();
    } while(s <= 0);

    srand(time(NULL));

    Taquin_init(&t, w, h);
    Taquin_shuffle(&t, s);
    CLEAR();
    Taquin_print(&t);
    printf("Your command (Enter 'h' for help) : ");
    for(;;) {
        c = getchar();
        if(c=='\n' || c==EOF)
            continue;
        Taquin_exec(&t, c);
        if(Taquin_is_resolved(&t))
            break;
        CLEAR();
        Taquin_print(&t);
        if(c == 'h') {
            putchar('\n');
            puts(TAQUIN_HOWTOPLAY);
            printf("Your command (Enter 'h' for help) : ");
            continue;
        }
        printf("Your command (Enter 'h' for help) : ");
        if(c == 'q') {
            Taquin_deinit(&t);
            return EXIT_SUCCESS; 
        }
    }
    CLEAR();
    Taquin_print(&t);
    Taquin_deinit(&t);

    putchar('\n');
    puts(w==2 && h==2 
        ? "Well, that was easy..." 
        : "You rock ! See you next time...");

    return EXIT_SUCCESS;
}
