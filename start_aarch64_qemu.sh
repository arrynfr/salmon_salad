cargo clean && 
cargo build && 
rust-objcopy -O binary target/aarch64-unknown-none/debug/salmon_salad target/aarch64-unknown-none/debug/salmon_salad.o && 
qemu-system-aarch64 -m 2048M -cpu cortex-a76 -smp 4 -M virt,gic-version=3 -device virtio-net,netdev=net0 -netdev user,id=net0 -serial mon:stdio -device ramfb -usb -device usb-ehci,id=ehci -device usb-kbd -device loader,addr=0x40800000,cpu-num=0,file=target/aarch64-unknown-none/debug/salmon_salad.o
