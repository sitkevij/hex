#!/bin/sh

HX_BIN="../../target/debug/hx"
# param ordering 1
$HX_BIN -ar tiny.txt
# param ordering 2
$HX_BIN tiny.txt -ar
# binary output column width 4
$HX_BIN -c4 -fb alphanumeric.txt
# missing len param
$HX_BIN --len tiny.txt
# missing file name
# $HX_BIN missing-file
# simulate broken pipe
dd if=/dev/random bs=512 count=10 | RUST_BACKTRACE=1 $HX_BIN | head -n 10
