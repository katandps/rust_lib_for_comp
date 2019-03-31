# develop_rust

# コンパイル

```:bash
docker-compose exec develop_rust rustc {filename}.rs
```

# 実行

```
docker-compose exec develop_rust filename
```

# IntelliJ で必要な機能

## Command Line Tool Support
- `./bin` 以下のファイルに対してエイリアスをつけておく