//! unwrap / expect / `?` の使い分け。
//!
//! - unwrap(): 本番コードでは原則使わない
//! - expect("理由"): 不変条件の根拠を文字列で残す（typestate 的保証など）
//! - ?: 呼び出し元へエラーを伝播
//!
//! 「絶対に Some」が分かっている文脈でも、なぜそう言えるかをメッセージで残す。

use std::num::ParseIntError;

fn parse_first_token(s: &str) -> Result<i32, ParseIntError> {
    // ? でエラーを呼び出し元に伝播
    let token = s.split_whitespace().next().unwrap_or("0");
    let n: i32 = token.parse()?;
    Ok(n)
}

fn pop_known_value(stack: &mut Vec<i32>) -> i32 {
    // 呼び出し規約として「事前に最低 1 要素ある」を保証している場面の例。
    // unwrap よりも expect で根拠を残す。
    stack.pop().expect("caller guarantees the stack is non-empty")
}

fn main() {
    println!("parsed: {:?}", parse_first_token("42 hello"));
    println!("err   : {:?}", parse_first_token("not-a-num"));

    let mut s = vec![1, 2, 3];
    let v = pop_known_value(&mut s);
    println!("popped: {v}");
}
