#!/bin/bash

# init
project_dir=$(dirname "$0")
bin_dir=$project_dir/target/riscv64gc-unknown-none-elf/release/
echo "Set path = $bin_dir"
#
cargo clean
cargo build --release

rust-objcopy --binary-architecture=riscv64 $bin_dir/tios --strip-all -O binary $bin_dir/tios.bin
#
echo done.
