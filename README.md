[![CI](https://github.com/katandps/rust_lib_for_comp/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/katandps/rust_lib_for_comp/actions/workflows/rust.yml)
[![verification](https://img.shields.io/badge/Doc-GitHubPages-brightgreen)](https://library.katand.net/)
[![rustdoc](https://img.shields.io/badge/Doc-GitHubPages-brightgreen)](https://library.katand.net/solver/)
[![coverage](https://img.shields.io/badge/Doc-GitHubPages-brightgreen)](https://library.katand.net/coverage/)

# Rust Library for Competitive Programming

競技プログラミングで使用できるライブラリ及び、各種サイトに参加するためのテンプレート
ディレクトリ構成は作者の都合により変わることがあります

## About

各ライブラリについて crate にしてある
crate の依存関係は Cargo.toml に記載している

master に push するとテスト/静的解析/Doc 生成が走る

## Usage

### Export snippets

#### Install

`cargo-snippet` `oj-verify` を使用しています。

```sh
rustup component add rustfmt
cargo install cargo-snippet --features="binaries"
```

```sh
sudo apt update
sudo apt install python3-pip
pip3 install online-judge-verify-helper
```

#### Export

テスト/verify を行った後、snippet を指定したフォルダに格納します

```
make build
```

### Benchmark

criterion によるベンチマークに対応しています。

#### Install

#### bench

```
cargo bench
```
