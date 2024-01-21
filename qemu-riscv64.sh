#!/usr/bin/env bash
set -e 

cargo build --target riscv64gc-unknown-none-elf
rust-objcopy -O binary target/riscv64gc-unknown-none-elf/debug/salmon_salad target/riscv64gc-unknown-none-elf/debug/salmon_salad.o

qemu-system-riscv64 -S -s \
	-bios none \
	-m 2048M \
	-cpu rv64 \
	-M virt \
	-nographic \
	-serial mon:stdio \
	-kernel target/riscv64gc-unknown-none-elf/debug/salmon_salad.o
