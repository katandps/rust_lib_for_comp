# Rust Library for Competitive Programming

[![Rust](https://github.com/katandps/rust_lib_for_comp/actions/workflows/rust.yml/badge.svg)](https://github.com/katandps/rust_lib_for_comp/actions/workflows/rust.yml)
[![document](https://img.shields.io/badge/Doc-GitHubPages-brightgreen)](https://library.katand.net/)
[![codecov](https://codecov.io/gh/katandps/rust_lib_for_comp/graph/badge.svg?token=MQ3QNO200B)](https://codecov.io/gh/katandps/rust_lib_for_comp)

## About

競技プログラミングで使用できるライブラリ及び、各種サイトに参加するためのテンプレート

## Usage

Makefileを見るべし

### Export snippets

#### Install

`cargo-snippet` `competitive-verifier` を使用しています。

##### Install cargo-snippet

```sh
rustup component add rustfmt
cargo install cargo-snippet --features="binaries"
```

##### Install competitive-verifier

```sh
pip install competitive-verifier
```

#### Export

テスト/verify を行った後、snippet を指定したフォルダに格納します

```sh
make build
```
