[![Rust](https://github.com/katandps/rust_lib_for_comp/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/katandps/rust_lib_for_comp/actions/workflows/rust.yml)
[![rustdoc](https://img.shields.io/badge/Doc-GitHubPages-brightgreen)](https://katandps.github.io/rust_lib_for_comp/rust_lib_for_comp)

# Rust Library for Competitive Programming

競技プログラミングで使用できるライブラリ及び、各種サイトに参加するためのテンプレート

## About

masterにpushするとテスト/静的解析/Doc生成が走る

## Usage

### Export snippets

#### Install
`cargo-snippet`を使用しています。
```sh
rustup component add rustfmt
cargo install cargo-snippet --features="binaries"
```

#### Export
```
cargo snippet
```

### Benchmark
criterionによるベンチマークに対応しています。

#### Install
#### bench
```
cargo bench
```