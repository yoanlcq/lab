#pragma once

#define NK_INCLUDE_FIXED_TYPES
#define NK_INCLUDE_VERTEX_BUFFER_OUTPUT
#define NK_INCLUDE_FONT_BAKING
#define NK_INCLUDE_DEFAULT_FONT // Embeds ProggyClean.ttf on stack ??
#define NK_INCLUDE_COMMAND_USERDATA
#define NK_ZERO_COMMAND_MEMORY
// #define NK_INPUT_MAX 4096 // Max number of text bytes an nk_input struct can accept
/*
// Idea : use __builtin_memcpy and al. instead ?
#include <string.h>
#define NK_MEMSET memset
#define NK_MEMCPY memcpy
#include <math.h>
#define NK_SQRT sqrt
#define NK_COS cosf
#define NK_SIN sinf
#define NK_STRTOD strtod
#define NK_DTOA dtoa
*/
// TODO: Allow this to be customized ?
#define NK_TEXTEDIT_UNDOSTATECOUNT     99
#define NK_TEXTEDIT_UNDOCHARCOUNT      999
