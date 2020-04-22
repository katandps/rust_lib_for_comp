# Rustで競技プログラミングのリポジトリ

作成したライブラリ/解いた問題の集積所

## 運用

演習時は `atcoder/src/bin` 以下にソースを置く
終わったら `atcoder` 以下に `try{yyyymmdd}` の名前でバイナリクレートを作り、bin以下に入れてコミットする

ディレクトリ構造
```
try{yyyymmdd}
  - src/
    - bin/
  - Cargo.toml
```