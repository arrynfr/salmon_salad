#!/usr/bin/env bash
set -e

cargo build --target x86_64-unknown-uefi

qemu-system-x86_64 \
	-m 2048 \
	-nographic \
	-bios ovmf/OVMF_x86_64.fd \
	-device driver=e1000,netdev=n0 \
	-netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=salmon_salad.efi
