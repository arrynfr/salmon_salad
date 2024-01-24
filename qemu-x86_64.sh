#!/usr/bin/env bash
set -e

#cargo build --target x86_64-unknown-uefi
cargo run --target x86_64-unknown-uefi --features uefi

