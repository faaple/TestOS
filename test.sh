#!/bin/bash

BOOTLOADER=../2025s-rcore-faaple/bootloader/rustsbi-qemu.bin
KERNEL_BIN=target/riscv64gc-unknown-none-elf/release/os
KERNEL_ENTRY_PA=0x80200000

cargo build --release
rust-objcopy --binary-architecture=riscv64 "$KERNEL_BIN" --strip-all -O binary "$KERNEL_BIN".bin
qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios "$BOOTLOADER" \
            -device loader,file="$KERNEL_BIN".bin,addr="$KERNEL_ENTRY_PA"