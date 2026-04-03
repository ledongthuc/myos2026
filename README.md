# myos2026

My OS but in 2026

After years, I'm write an operating system from scratch again. Let see how far I can get.

## Development setup

### Prerequires

 - https://www.qemu.org/
 - Setup bootloader requirement

```
cargo install bootimage
rustup component add llvm-tools-preview
```

### Quickstart

We need to run with Rust nightly:

```
rustup override set nightly
```

In Rust nightly, we also need rust-src

```
rustup component add rust-src
```

and run it (build + start QEMU):

```
cargo run
```

### Alternative: Build only

```
cargo build
```

### Alternative: Build and create bootimage

```
cargo bootimage
```

### Alternative: Start QEMU with a built image

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-myos2026/debug/bootimage-myos2026.bin
```
