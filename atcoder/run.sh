#!/bin/bash
cd `dirname $0`

if [ $# -ne 1 ]; then
  echo "問題が指定されていません"
  exit 1
fi

time RUST_BACKTRACE=1 cargo run --bin task_$1 < sample/$1.txt