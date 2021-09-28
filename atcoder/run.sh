#!/bin/bash
cd `dirname $0`

if [ $# -ne 1 ]; then
  echo "問題が指定されていません"
  exit 1
fi

cargo build --release --bin task_$1
time RUST_BACKTRACE=1 ./target/release/task_$1 < sample/$1.txt >/dev/null