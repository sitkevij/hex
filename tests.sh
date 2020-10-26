#!/bin/sh

# param ordering 1
target/debug/hx -ar tests/files/tiny.txt
# param ordering 2
target/debug/hx tests/files/tiny.txt -ar
# missing len param
target/debug/hx --len tests/files/tiny.txt
# missing file name
target/debug/hx missing-file
# simulate broken pipe
dd if=/dev/random bs=512 count=10 | RUST_BACKTRACE=1 target/debug/hx | head -n 10
