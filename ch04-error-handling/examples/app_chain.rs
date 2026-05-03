//! アプリケーション層の典型: anyhow で動的に受け、Context でメッセージを足す。
//!
//! - `.context("...")` は静的メッセージ用（allocation なし）
//! - `.with_context(|| format!("..."))` はエラー時のみ format を評価する
//!   ホットパスでは後者を使う

use anyhow::{Context, Result, bail};

fn read_config_text(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config from {path}"))
}

fn parse_threshold(text: &str) -> Result<u32> {
    let line = text
        .lines()
        .next()
        .context("config file is empty")?;
    let n: u32 = line
        .trim()
        .parse()
        .with_context(|| format!("threshold must be u32, got: {line:?}"))?;
    if n == 0 {
        bail!("threshold must be > 0");
    }
    Ok(n)
}

fn run() -> Result<u32> {
    let text = read_config_text("/no/such/file")?;
    parse_threshold(&text)
}

fn main() {
    if let Err(e) = run() {
        // {:?} でエラーチェーン全体を表示
        eprintln!("error: {e:?}");
    }
}
