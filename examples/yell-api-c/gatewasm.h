#pragma once

#include <msgpack.h>

#define MAX_MSG_SIZE 2048

typedef struct gatewasm_request {
  char input[1024];
} gatewasm_request;

typedef struct gatewasm_response {
  char output[1024];
} gatewasm_response;

gatewasm_request gatewasm_get_request(int ptr);
int gatewasm_build_response(gatewasm_response *response);
