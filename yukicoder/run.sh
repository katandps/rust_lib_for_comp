#!/bin/bash
cd `dirname $0`

RUST_BACKTRACE=1
echo "" >> sample/$1.txt
cargo run --release < sample.txt