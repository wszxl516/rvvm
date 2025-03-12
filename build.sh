#!/bin/sh
riscv64-linux-gnu-gcc code.s  -o code.elf -nostdlib -static -Tld.ld -fPIC
riscv64-linux-gnu-objcopy -O binary --only-section=.text --only-section=.data code.elf code.bin