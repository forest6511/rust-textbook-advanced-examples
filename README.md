# 現場で使えるRust言語実践テクニック サンプルコード

書籍『[現場で使えるRust言語実践テクニック — 並行処理・パフォーマンス最適化・トレイト設計の実装パターン](https://www.amazon.co.jp/dp/B0GZJ3C983)』（森川 陽介 著 / Amazon Kindle）の章別サンプルコードリポジトリ。

Vol.1（[Rust言語の教科書](https://www.amazon.co.jp/dp/B0GSHJ3PTQ) / [サンプルコード](https://github.com/forest6511/rust-textbook-examples)）の続編にあたる中級者向け実践書のコンパニオンリポジトリです。

## 動作要件

- **Rust**: 1.85.0 以上（async closures が必要）
- **Edition**: 2024
- **Cargo resolver**: 3

`rust-toolchain.toml` で 1.95 を指定しています。`rustup` がインストール済みなら自動で切り替わります。

## 章構成

| ディレクトリ | 章 | 主要トピック |
|---|---|---|
| `ch01-ownership/` | 1 | 所有権の応用パターン（Cow/Rc/RefCell） |
| `ch02-trait-design/` | 2 | トレイト設計の判断基準（dyn vs impl） |
| `ch03-generics/` | 3 | ジェネリクスとトレイト境界（HRTB/PhantomData） |
| `ch04-error-handling/` | 4 | エラーハンドリング応用（thiserror/anyhow） |
| `ch05-concurrency/` | 5 | 並行処理の基礎（thread/Mutex/Arc） |
| `ch06-async-tokio/` | 6 | async/await と Tokio 入門 |
| `ch07-rayon/` | 7 | rayon による並列処理 |
| `ch08-paradigm/` | 8 | 3パラダイム使い分け（thread/async/rayon 比較ベンチ） |
| `ch09-criterion/` | 9 | criterion ベンチマーク実践 |
| `ch10-memory-opt/` | 10 | メモリ最適化テクニック |
| `ch11-web-crawler/` | 11 | 実プロジェクト 並行Webクローラ |

## 使い方

```bash
git clone https://github.com/forest6511/rust-textbook-advanced-examples.git
cd rust-textbook-advanced-examples

# 全章をビルド
cargo build --workspace

# 特定の章を実行
cargo run -p ch06-async-tokio --bin <example-name>

# ベンチマーク実行
cargo bench -p ch09-criterion

# テスト実行
cargo test --workspace

# Lint
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

## CI

`.github/workflows/ci.yml` で以下を検証しています:

- Rust 1.85 / 1.94 / stable のマトリクスビルド
- `cargo build --workspace`
- `cargo test --workspace`
- `cargo clippy -- -D warnings`
- `cargo fmt -- --check`

## 関連書籍

- **Vol.1**: [Rust言語の教科書](https://www.amazon.co.jp/dp/B0GSHJ3PTQ) — 入門書（基礎・所有権・基本型）
- **Vol.2**: 現場で使えるRust言語実践テクニック（本書、出版予定）

## ライセンス

MIT License。詳細は [LICENSE](LICENSE) を参照。

## 著者

森川 陽介 (forest6511)
