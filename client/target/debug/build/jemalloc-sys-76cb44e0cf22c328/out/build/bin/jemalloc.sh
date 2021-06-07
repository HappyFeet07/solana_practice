#!/bin/sh

prefix=/Users/jacktseng/work/rust/SPL/client/target/debug/build/jemalloc-sys-76cb44e0cf22c328/out
exec_prefix=/Users/jacktseng/work/rust/SPL/client/target/debug/build/jemalloc-sys-76cb44e0cf22c328/out
libdir=${exec_prefix}/lib

DYLD_INSERT_LIBRARIES=${libdir}/libjemalloc.2.dylib
export DYLD_INSERT_LIBRARIES
exec "$@"
