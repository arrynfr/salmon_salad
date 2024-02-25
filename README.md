# salmon_salad
Small operating system for to learn about os-dev

# Dependencies
I want to keep the dependency list as small as possible to explore everything by implementing it myself and having a self contained, easy to build program.  
The current dependency list is:  

- rust nightly
- qemu for any system you wanna run
- gdb for debugging

# Compiling
We currently support these platforms:
```
x86_64-unknown-uefi
aarch64-unknown-uefi
aarch64-unknown-none
aarch64-unknown-none-softfloat
riscv64gc-unknown-none-elf
```

The gist of compiling the program is to just choose one target from the list and run:
```
cargo build --target YOUR_TARGET
```

For `*-uefi` targets it's important to additionally pass the feature flag uefi to the compiler:
```
cargo build --target YOUR_TARGET --features uefi
```

# Running
Replace the ``build`` in the build command by ``run``.