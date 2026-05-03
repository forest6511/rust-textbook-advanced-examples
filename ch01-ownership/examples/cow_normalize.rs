use std::borrow::Cow;

// 入力に変更が必要なときだけ String を確保し、不要なら借用のまま返す。
fn normalize(input: &str) -> Cow<'_, str> {
    if input.chars().all(|c| !c.is_uppercase()) {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(input.to_lowercase())
    }
}

fn main() {
    let already = normalize("hello");
    let needs = normalize("Hello");

    println!("already: {already}");
    println!("needs:   {needs}");

    assert!(matches!(already, Cow::Borrowed(_)));
    assert!(matches!(needs, Cow::Owned(_)));
}
