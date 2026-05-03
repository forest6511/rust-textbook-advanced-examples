//! ライブラリ層 (thiserror) のエラーを、アプリ層 (anyhow) で受けるパターン。
//!
//! anyhow::Error は std::error::Error を実装する任意の型を吸収するので、
//! lib 層の具象エラーを `?` でそのまま `anyhow::Result` に流せる。
//! アプリ層は context を足し、必要なら downcast_ref で具象型を拾い直す。

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibError {
    #[error("permission denied")]
    PermissionDenied,
    #[error("internal i/o error")]
    Io(#[from] std::io::Error),
}

fn lib_call(allow: bool) -> Result<String, LibError> {
    if !allow {
        return Err(LibError::PermissionDenied);
    }
    Ok("ok".to_string())
}

fn app_call(allow: bool) -> Result<String> {
    let s = lib_call(allow).context("calling lib_call")?;
    Ok(s)
}

fn main() {
    match app_call(false) {
        Ok(s) => println!("ok: {s}"),
        Err(e) => {
            // downcast_ref で具象 LibError を取り出す
            if let Some(LibError::PermissionDenied) = e.downcast_ref::<LibError>() {
                eprintln!("specific: permission denied");
            } else {
                eprintln!("other: {e}");
            }
        }
    }
}
