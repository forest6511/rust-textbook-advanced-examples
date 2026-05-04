//! 並行 Web クローラの core ライブラリ。
//!
//! Ch.11 の本文と対応する構成要素を提供します。
//! - [`fetch`]: HTTP クライアントとフェッチ関数
//! - [`parse`]: HTML パースとリンク抽出
//! - [`limiter`]: global + per-host のレート制限
//! - [`queue`]: visited URL の重複排除
//! - [`storage`]: SQLite Single-Writer スレッド
//! - [`runner`]: すべてを組み立てる `run` 関数
pub mod fetch;
pub mod limiter;
pub mod parse;
pub mod queue;
pub mod runner;
pub mod storage;
pub mod types;

pub use runner::{Config, run};
