#!/usr/bin/env bash
set -e

cargo build --target x86_64-unknown-uefi
qemu-system-x86_64 \
	-m 4096 \
	-nographic \
	-bios OVMF.fd \
	-device driver=e1000,netdev=n0 \
	-netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=salmon_salad.efi
