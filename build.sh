#!/bin/sh
riscv64-linux-gnu-gcc code.s  -o code.elf -nostdlib -static -Tld.ld
rust-objcopy -O binary code.elf code.bin