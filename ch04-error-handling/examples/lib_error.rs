//! ライブラリ層の典型: thiserror で具象エラー型を提供する。
//!
//! 呼び出し側が `match` で分岐できる enum を返すのが lib 層の責務。
//! `#[from]` で `?` の自動変換を成立させ、`#[source]` でエラーチェーンを構築する。

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config file not found: {path}")]
    NotFound { path: String },

    #[error("failed to read config")]
    Io(#[from] std::io::Error),

    #[error("invalid port number: {raw:?}")]
    InvalidPort {
        raw: String,
        #[source]
        source: ParseIntError,
    },
}

pub fn load_port(text: &str) -> Result<u16, ConfigError> {
    text.trim().parse::<u16>().map_err(|e| ConfigError::InvalidPort {
        raw: text.to_owned(),
        source: e,
    })
}

fn main() {
    match load_port("not-a-number") {
        Ok(p) => println!("port: {p}"),
        Err(e) => {
            println!("error: {e}");
            // source() でチェーンを辿る
            if let Some(src) = std::error::Error::source(&e) {
                println!("caused by: {src}");
            }
        }
    }
}
