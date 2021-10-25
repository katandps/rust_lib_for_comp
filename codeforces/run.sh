#!/bin/bash
cd `dirname $0`
set -e

RUST_BACKTRACE=1 cargo run < sample.txt
cargo build --release &> /dev/null && time ./target/release/library_checker < sample.txt &> /dev/null
