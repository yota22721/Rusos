#!/bin/bash
set -xue

# QEMUのファイルパス
QEMU=qemu-system-riscv64
KERNEL=target/riscv64gc-unknown-none-elf/debug/Rusos

cargo build --target riscv64gc-unknown-none-elf

# QEMUを起動
$QEMU -machine virt -bios none -nographic -serial mon:stdio --no-reboot -kernel $KERNEL
