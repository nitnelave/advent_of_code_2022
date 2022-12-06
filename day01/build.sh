#!/bin/bash

cargo rustc --release -- --emit obj
ld --build-id=none --no-eh-frame-hdr -n -N --no-dynamic-linker -m elf_x86_64 -no-pie -znoexecstack --strip-all -nostartfiles -Bstatic -o target/release/day01 target/release/deps/day01-*.o target/release/deps/libcompiler_builtins-*.rlib --gc-sections --nmagic
objcopy -R .eh_frame -R .got.plt -R .dynamic -R .dynstr -R .dynsym target/release/day01 day01
