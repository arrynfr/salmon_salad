DEBUG="-d mmu,guest_errors,unimp"
SMP="-smp 4"
GDB="-s -S"
#VIRTIO="-device virtio-net,netdev=net0 -device virtio-rng-pci"
DUMP="-object filter-dump,id=f1,netdev=u1,file=dump.pcap"
#DTB=",dumpdtb=virt.dtb"
#COMP="/home/arryn/git/qemu/build/"
NET="-netdev bridge,br=virbr0,id=u1 -device e1000,netdev=u1"

cargo clean && 
cargo build && 
rust-objcopy -O binary target/aarch64-unknown-none/debug/salmon_salad target/aarch64-unknown-none/debug/salmon_salad.o && 
"$COMP"qemu-system-aarch64 $GDB $DEBUG -m 2048M -cpu cortex-a76 $SMP -M virt,gic-version=3$DTB $VIRTIO -device virtio-tablet-pci \
$NET $DUMP -serial mon:stdio -device ramfb -usb -device usb-ehci,id=ehci -device usb-kbd \
-device virtio-keyboard-pci -device virtio-mouse-pci -device loader,addr=0x40800000,cpu-num=0,file=target/aarch64-unknown-none/debug/salmon_salad.o
