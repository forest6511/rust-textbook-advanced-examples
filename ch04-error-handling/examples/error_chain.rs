//! エラーチェーンを source() で順に辿って全て表示する。
//!
//! `std::error::Error::source()` で次の層を取得し、None になるまで辿る。
//! 各層で thiserror の `#[source]` または `#[from]` で source を繋いでおけば、
//! チェーン全体を取り出して構造化ログに出せる。
//! (`Error::sources()` イテレータは現状 unstable feature。stable では自前でループする)

use std::error::Error as StdError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("middle layer error")]
pub struct MiddleError {
    #[source]
    source: std::io::Error,
}

#[derive(Debug, Error)]
pub enum TopError {
    #[error("top layer: {context}")]
    Wrap {
        context: String,
        #[source]
        source: MiddleError,
    },
}

fn make_error() -> TopError {
    let io_err = std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "denied by policy",
    );
    let middle = MiddleError { source: io_err };
    TopError::Wrap {
        context: "loading user profile".into(),
        source: middle,
    }
}

fn main() {
    let err = make_error();
    println!("error: {err}");

    // source() を None になるまで辿る
    let mut current: Option<&dyn StdError> = err.source();
    let mut i = 0;
    while let Some(src) = current {
        println!("  caused by ({i}): {src}");
        current = src.source();
        i += 1;
    }
}
