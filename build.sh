#!/bin/sh
kldunload ./hello.ko
set -e
export CARGO_TARGET_DIR=/tmp/target
LIBCLANG_PATH=/usr/local/llvm90/lib cargo build -Z build-std=core,alloc --target x86_64-kernel-freebsd.json
cd objects
# ignore errors here
rm * || true
ar -xv "${CARGO_TARGET_DIR}/x86_64-kernel-freebsd/debug/libhello.a"
cd ..
make
kldload ./hello.ko
