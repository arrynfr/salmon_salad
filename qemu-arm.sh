#!/usr/bin/env bash
cargo build --target aarch64-unknown-uefi
qemu-system-aarch64 -m 4096M -cpu host -M virt,accel=hvf \
        -bios OVMF_aarch64.fd -device virtio-gpu-pci \
        -device virtio-net,netdev=net0 \
	-netdev user,id=net0,tftp=target/aarch64-unknown-uefi/debug,bootfile=salmon_salad.efi
