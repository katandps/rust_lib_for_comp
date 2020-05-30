#!/bin/bash

if [ $# -ne 1 ]; then
  echo "問題番号を指定してください"
  echo "example: \$bash run.sh a"
fi

NUM=$1

echo "cargo build --release --bin $NUM"
cargo build --release --bin $NUM
echo "time ./target/release/$NUM < sample/$NUM.txt"
time ./target/release/$NUM < sample/$NUM.txt
