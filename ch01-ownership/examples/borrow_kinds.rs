// 関数引数で「所有権を取る」「不変借用」「可変借用」を使い分ける。
fn consume(name: String) -> usize {
    // 所有権を奪うので、呼び出し後 name は使えない。
    name.len()
}

fn borrow(name: &str) -> usize {
    // &str は &String / リテラル / Box<str> いずれからも作れる。
    name.len()
}

fn append_world(buf: &mut String) {
    buf.push_str(" world");
}

fn main() {
    let owned = String::from("hello");
    let _ = borrow(&owned);
    let _ = borrow("plain literal");

    let mut buf = String::from("hello");
    append_world(&mut buf);
    assert_eq!(buf, "hello world");

    let len = consume(owned);
    println!("consumed length = {len}");
}
