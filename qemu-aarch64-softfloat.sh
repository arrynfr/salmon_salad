#!/usr/bin/env bash
set -e 

cargo build --target aarch64-unknown-none-softfloat
rust-objcopy -O binary target/aarch64-unknown-none-softfloat/debug/salmon_salad target/aarch64-unknown-none-softfloat/debug/salmon_salad.o

qemu-system-aarch64 -S -s \
	-m 4096M \
	-cpu cortex-a57 \
	-M virt \
	-nographic \
	-serial mon:stdio \
	-kernel target/aarch64-unknown-none-softfloat/debug/salmon_salad.o

