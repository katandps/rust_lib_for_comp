#!/bin/bash
cd `dirname $0`
set -e

if [ $# -ne 1 ]; then
  echo "問題が指定されていません"
  exit 1
fi

RUST_BACKTRACE=1 cargo run --bin task_$1 < sample/$1.txt
cargo build --bin task_$1 --release &> /dev/null && time ./target/release/task_$1 < sample/$1.txt &> /dev/null
