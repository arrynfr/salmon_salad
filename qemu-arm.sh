#!/usr/bin/env bash
set -e 

cargo build --target aarch64-unknown-uefi


#if[ "$(uname -m)" == 'aarch64' ]
#then
#	case "$(uname -s)" in
#		Linux*)		accelerator=kvm;;
#		Darwin*)	accelerator=hvf;;
#	esac
#fi

qemu-system-aarch64 \
	-m 4096M \
	-cpu cortex-a57 \
	-M virt \
    -bios OVMF_aarch64.fd \
    -device virtio-net,netdev=net0 \
	-nographic \
	-netdev user,id=net0,tftp=target/aarch64-unknown-uefi/debug,bootfile=salmon_salad.efi
