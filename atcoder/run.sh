#!/bin/bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
echo $DIR
cd $DIR

if [ $# -ne 1 ]; then
  echo "問題番号を指定してください"
  echo "example: \$bash run.sh a"
fi

NUM=$1
echo "cargo build --color=always --package atcoder --bin $NUM"
cargo build --color=always --package atcoder --bin $NUM
echo "time -p ./target/debug/$NUM < sample/$NUM.txt"
RUST_BACKTRACE=1 time -p ./target/debug/$NUM < sample/$NUM.txt