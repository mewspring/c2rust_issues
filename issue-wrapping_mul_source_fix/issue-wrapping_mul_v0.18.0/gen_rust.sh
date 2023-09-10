#!/bin/bash

# Generate compile_commands.json
make clean
intercept-build make

# Transpile C to Rust

# NOTE: add include path of stddef.h and stdarg.h (/usr/lib/include).
rm -rf out
c2rust transpile --emit-build-files compile_commands.json -o out --binary main -- -I/usr/lib/include
