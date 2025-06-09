#!/bin/bash
cargo build
qemu-system-x86_64 -drive format=raw,file=target/x86_64-rust-os/debug/bootimage-rust-os.bin