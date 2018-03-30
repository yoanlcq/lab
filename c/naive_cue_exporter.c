/*
 * A little program for extracting tracks from a big ".flac" file, 
 * given a ".cue" file. It makes serveral assumptions and might not take
 * all cases into account (TL:DR; it might not work for you).
 *
 */

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <dirent.h>

#include <sndfile.h>

struct cuetrack {
    unsigned trackno;
    char title[256];
    char performer[256];
    uint64_t index_ms;
};

struct cue {
    char performer[256];
    char title[256];
    char filename[256];
    unsigned numtracks;
    struct cuetrack *tracks;
};

char *read_to_end(FILE *stream) {
    size_t nbytes, total;
    char buf[4096];
    char *res = NULL;
    for(total=0 ; ; total += nbytes) {
        nbytes = fread(buf, 1, 4096, stream);
        if(nbytes <= 0)
            break;
        res = realloc(res, total+nbytes);
        memcpy(res+total, buf, nbytes);
    }
    return res;
}

void cue_from_file(struct cue *cue, const char *filename) {

    FILE *cuefile;
    char *cuebuf, *token;
    unsigned trackno;

    cuefile = fopen(filename, "r");
    if(!cuefile) {
        fprintf(stderr, "Couldn't open \"%s\". Sorry.\n", filename);
        exit(EXIT_FAILURE);
    }
    cuebuf = read_to_end(cuefile);
    fclose(cuefile);


    struct cuetrack *track = NULL;
    cue->numtracks = 0;
    cue->tracks = NULL;

    token = strtok(cuebuf, " \r\n");

    while(token) {

        if(!strcmp(token, "PERFORMER")) {
            strncpy(track ? track->performer : cue->performer, strtok(NULL, "\""), 256);
        } else if(!strcmp(token, "TITLE")) {
            strncpy(track ? track->title : cue->title, strtok(NULL, "\""), 256);
        } else if(!strcmp(token, "FILE")) {
            strncpy(cue->filename, strtok(NULL, "\""), 256);
        } else if(!strcmp(token, "TRACK")) {
            ++(cue->numtracks);
            cue->tracks = realloc(cue->tracks, cue->numtracks*sizeof(struct cuetrack));
            track = cue->tracks + cue->numtracks - 1;
            track->trackno = strtoul(strtok(NULL, " \r\n"), NULL, 10);
        } else if(!strcmp(token, "INDEX")) {
            strtok(NULL, " \r\n");
            track->index_ms = 60*1000*strtoul(strtok(NULL, ":"), NULL, 10);
            track->index_ms += 1000*strtoul(strtok(NULL, ":"), NULL, 10);
            track->index_ms += strtod(strtok(NULL, " \r\n"), NULL)*1000.0/100.0;
        }

        token = strtok(NULL, " \r\n");
    }

    free(cuebuf);
}


#ifdef _WIN32
#define clear_console_screen() system("cls")
#else
#define clear_console_screen() printf("\033c");
#endif

void export_cue_libsndfile(struct cue *cue, const char *cwd) {

    SNDFILE *source, *dest;
    SF_INFO srcinfo, dstinfo;
    char trackno_str[4], *dest_name, *src_path;
    uint64_t nframes;
    int64_t cnt;
    unsigned i;
    short *frames;


    asprintf(&src_path, "%s/%s", cwd, cue->filename);
    printf("Now exporting \"%s\".\n", src_path);
    source = sf_open(src_path, SFM_READ, &srcinfo);
    free(src_path);
    if(srcinfo.format & SF_FORMAT_SUBMASK != SF_FORMAT_PCM_16) {
        fprintf(stderr, "Not 16-bit format. Sorry.\n");
        sf_close(source);
        return;
    }
    memcpy(&dstinfo, &srcinfo, sizeof(SF_INFO));
    /*
    dstinfo.format = SF_FORMAT_OGG 
                   | SF_FORMAT_VORBIS 
                   | (srcinfo.format & SF_FORMAT_ENDMASK);
    */
    dstinfo.format = SF_FORMAT_AIFF 
                   | SF_FORMAT_PCM_16
                   | SF_ENDIAN_BIG;
 
    if(!sf_format_check(&dstinfo)) {
        fprintf(stderr, "Output file format is invalid :"
                " sf_format_check() returned FALSE.\n");
        sf_close(source);
        return;
    }

    unsigned blksiz;
    for(blksiz = 1<<16 ; ; blksiz>>=1) {
        frames = malloc(sizeof(short)*dstinfo.channels*blksiz);
        if(frames) break; /* Don't put in condition. It's a do...while. */
    }

    for(i=0 ; i<cue->numtracks ; ++i) {
        asprintf(&dest_name, "%s/%u - %s.aiff", 
                cwd, cue->tracks[i].trackno, 
                strchr(cue->tracks[i].title, ':') ? "__Illegal_Title__" 
                                                 : cue->tracks[i].title);
        dest = sf_open(dest_name, SFM_WRITE, &dstinfo);
        sf_set_string(dest, SF_STR_TITLE, cue->tracks[i].title);
        sf_set_string(dest, SF_STR_ARTIST, cue->tracks[i].performer);
        sf_set_string(dest, SF_STR_ALBUM, cue->title);
        sprintf(trackno_str, "%u", cue->tracks[i].trackno);
        sf_set_string(dest, SF_STR_TRACKNUMBER, trackno_str);
        /* stuff */
        nframes = srcinfo.samplerate*cue->tracks[i].index_ms/1000;
        sf_seek(source, nframes, SEEK_SET);
        if(i==cue->numtracks-1) {
            for(;;) {
                clear_console_screen();
                printf("Converting track %u/%u :\n"
                        "to %s\n"
                        "(reading to end of file)\n"
                        "From \"%s\"", 
                        i+1, cue->numtracks, dest_name, cue->filename);
                cnt = sf_readf_short(source, frames, blksiz);
                if(cnt <= 0)
                    break;
                sf_writef_short(dest, frames, cnt);
            }
        } else {
            nframes = (srcinfo.samplerate*cue->tracks[i+1].index_ms/1000)
                -nframes-srcinfo.samplerate*2;
            for(cnt=0 ; cnt<nframes ; cnt += blksiz) {
                clear_console_screen();
                printf("Converting track %u/%u :\n"
                        "to %s\n"
                        "(%lf%%)\n"
                        "From \"%s\"", 
                        i+1, cue->numtracks, dest_name,
                        100.0*(double)cnt/(double)nframes, cue->filename);
                sf_readf_short(source, frames, blksiz);
                sf_writef_short(dest, frames, blksiz);
            }
            cnt -= blksiz;
            sf_readf_short(source, frames, nframes-cnt);
            sf_writef_short(dest, frames, nframes-cnt);
        }
        sf_close(dest);
        free(dest_name);
    }
    puts("Done. Now cleaning up.");
    sf_close(source);
    free(frames);
}

void parse_directory(const char *dirpath) {
    DIR *dir;
    struct dirent *dentry;
    unsigned i;
    char *newpath, *extension;
    struct cue cue;
   
    dir = opendir(dirpath);
    if(!dir)
        return;
    for(;;) {
        dentry = readdir(dir);
        if(!dentry)
            break;
        if(dentry->d_name[0]=='.') {
            if(dentry->d_namlen==1 
            ||(dentry->d_namlen==2 && dentry->d_name[1]=='.'))
                continue;
        }

        asprintf(&newpath, "%s/%s", dirpath, dentry->d_name);
        extension = strrchr(dentry->d_name, '.');
        if(extension && !strcasecmp(extension+1, "cue")) {
            cue_from_file(&cue, newpath);
            export_cue_libsndfile(&cue, dirpath);
        } else {
            parse_directory(newpath);
        }
        free(newpath);
    }
    closedir(dir);

}

int main(int argc, char *argv[]) {
    DIR *dir;
    unsigned i;

    if(argc <= 1) {
        fputs("I need one or more directories "
              "to operate recursively on.\n", stderr);
        exit(EXIT_SUCCESS);
    }

    for(i=1 ; i<argc ; ++i) {
        dir = opendir(argv[i]);
        if(dir) {
            closedir(dir);
            parse_directory(argv[i]);
        } else
            fprintf(stderr, "Could not open \"%s\".\n", argv[i]);
    }
    return EXIT_SUCCESS;
}

