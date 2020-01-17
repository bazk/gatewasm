#include <emscripten.h>
#include <ctype.h> 

#include "gatewasm.h"

// Converts a string to uppercase
void uppercase(char *dst, const char *src) {
    char *s = (char*) src;
    char *d = (char*) dst;
    while (*s) {
        *d = toupper((unsigned char) *s);
        s++;
        d++;
    }
}

EMSCRIPTEN_KEEPALIVE
int handler(int ptr) {
    gatewasm_request request = gatewasm_get_request(ptr);

    gatewasm_response response;
    uppercase(response.output, request.input);
    strcat(response.output, "!!1!!!!");

    return gatewasm_build_response(&response);
}
