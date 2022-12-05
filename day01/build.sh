#!/bin/bash

cargo build --release
objcopy -R .eh_frame -R .got.plt -R .dynamic -R .dynstr -R .dynsym target/release/day01 day01
