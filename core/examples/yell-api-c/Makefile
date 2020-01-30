all: yell.wasm

yell.wasm: yell.c gatewasm.c
	emcc -O3 -Imsgpack-c/include -o yell.wasm msgpack-c/libmsgpackc.a gatewasm.c yell.c

clean:
	rm -f yell.wasm
