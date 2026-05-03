//! where 句が「ジェネリックパラメータの直接境界」では書けない 3 つのパターン。
//!
//! 1. 関連型 (associated type) の上に境界を載せる
//! 2. 非パラメータ型 (concrete type) の上に境界を載せる
//! 3. 同じ T に対して `T: Trait1` と `&T: Trait2` のように形を変える境界

use std::fmt::Display;

// 1. 関連型に境界: Iterator の Item が Display であることを要求する。
//    `<I: Iterator<Item: Display>>` 短縮形は安定機能だが、where のほうが伝わりやすい。
fn join_with_comma<I>(iter: I) -> String
where
    I: Iterator,
    I::Item: Display,
{
    let mut buf = String::new();
    for (i, item) in iter.enumerate() {
        if i > 0 {
            buf.push_str(", ");
        }
        buf.push_str(&item.to_string());
    }
    buf
}

// 2. 非パラメータ型 String に対して、T で組み立てた境界を載せる。
//    呼び出し側で T が決まってから「String: From<T>」が成立するかが決まる。
fn build_message<T>(value: T) -> String
where
    String: From<T>,
{
    let s: String = value.into();
    format!("[msg] {s}")
}

// 3. 借用形に対する境界: T 自身は Clone を要求しないが、&T が IntoIterator
//    であることを要求する。Vec<T> や HashSet<T> のジェネリック処理で頻出。
fn count_items<T>(collection: &T) -> usize
where
    for<'a> &'a T: IntoIterator,
{
    let mut n = 0;
    for _ in collection {
        n += 1;
    }
    n
}

fn main() {
    let nums = [1, 2, 3];
    println!("join: {}", join_with_comma(nums.iter()));
    println!("msg : {}", build_message("hello"));
    let owned = vec![10, 20, 30, 40];
    println!("len : {}", count_items(&owned));
}
