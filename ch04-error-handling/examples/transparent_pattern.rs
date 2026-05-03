//! `#[error(transparent)]` で外部 crate のエラーを素通しする。
//!
//! 自分で意味付けすべきエラーには使わない（情報が消える）。
//! 「上位レイヤーで意味を変えずに伝播するだけ」のケース限定。

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("user-facing: {0}")]
    UserFacing(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] std::num::ParseIntError),
}

fn parse_count(s: &str) -> Result<u32, AppError> {
    let n = s.parse::<u32>()?;
    Ok(n)
}

fn main() {
    match parse_count("abc") {
        Ok(n) => println!("count: {n}"),
        Err(e) => {
            // transparent の variant は内部エラーの Display と source をそのまま出す
            println!("error: {e}");
            if let Some(src) = std::error::Error::source(&e) {
                println!("source: {src}");
            }
        }
    }
}
