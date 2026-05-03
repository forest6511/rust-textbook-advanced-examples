//! HRTB (higher-ranked trait bounds) は「任意のライフタイム 'a に対して」を表す。
//!
//! `Fn(&str) -> &str` のように借用を取り回すクロージャを境界に書くとき、
//! `for<'a> Fn(&'a str) -> &'a str` の HRTB が必要になる。
//! 多くの場合は省略形でコンパイラが補完してくれるが、複数の借用が
//! 絡むケースでは明示的な `for<'a>` が判定の助けになる。

// 「どんなライフタイムの &str を渡しても、同じライフタイムの &str を返す」関数を受け取る。
// 暗黙形式: F: Fn(&str) -> &str でも同じ意味になる。
fn apply_explicit<F>(f: F, s: &str) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s).to_owned()
}

// 「複数の借用」が絡むと、暗黙形式では型推論が失敗するケースがある。
// for<'a> を明示することで「全ての 'a に対して」を要求できる。
fn apply_two<F>(f: F, a: &str, b: &str) -> (String, String)
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    (f(a).to_owned(), f(b).to_owned())
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    let head = apply_explicit(first_word, "hello world");
    println!("head: {head}");

    let (x, y) = apply_two(first_word, "rust async", "tokio runtime");
    println!("two : {x} / {y}");
}
