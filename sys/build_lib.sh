#!/bin/sh 
wasicc -c ../quickjs/libbf.c ../quickjs/cutils.c ../quickjs/libunicode.c ../quickjs/lib/wapper.c ../quickjs/libregexp.c ../quickjs/quickjs-libc.c -DCONFIG_BIGNUM=1 -DCONFIG_VERSION='"wasi"' -D_WASI_EMULATED_SIGNAL -lwasi-emulated-signal -O3
wasiar -r libquickjs.a *.o 
bindgen ../quickjs/lib/wapper.h --size_t-is-usize -o wasm32-wasi.rs -- -D__wasi__
# cp static-lib and binding.rs to quickjs-rs-wasi
# copy to out_dir
cp libquickjs.a $1
cp wasm32-wasi.rs $1
rm *.o 