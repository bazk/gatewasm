#include "gatewasm.h"

gatewasm_request gatewasm_get_request(int ptr) {
    gatewasm_request req;
    msgpack_zone mempool;
    msgpack_object obj;

    msgpack_zone_init(&mempool, 2048);
    msgpack_unpack((const char *) ptr, MAX_MSG_SIZE, NULL, &mempool, &obj);

    assert(obj.type == MSGPACK_OBJECT_MAP);

    msgpack_object_kv *attr = obj.via.map.ptr;
    assert(attr->key.type == MSGPACK_OBJECT_STR);
    assert(attr->val.type == MSGPACK_OBJECT_STR);
    assert(strncmp(attr->key.via.str.ptr, "input", attr->key.via.str.size) == 0);
    strncpy(req.input, attr->val.via.str.ptr, attr->val.via.str.size);

    msgpack_zone_destroy(&mempool);

    return req;
}

int gatewasm_build_response(gatewasm_response *response) {
    msgpack_sbuffer sbuf;
    msgpack_packer pk;

    msgpack_sbuffer_init(&sbuf);
    msgpack_packer_init(&pk, &sbuf, msgpack_sbuffer_write);

    msgpack_pack_map(&pk, 1);
    msgpack_pack_str(&pk, 6);
    msgpack_pack_str_body(&pk, "output", 6);
    msgpack_pack_str(&pk, strlen(response->output));
    msgpack_pack_str_body(&pk, response->output, strlen(response->output));

    return (int) sbuf.data;
}
