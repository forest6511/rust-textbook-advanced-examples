// 入力スライスから空白区切りでトークンを取り出すパーサ。
// 戻り値の寿命は構造体ではなく入力 source に紐づく。
struct Parser<'src> {
    source: &'src str,
    cursor: usize,
}

impl<'src> Parser<'src> {
    fn new(source: &'src str) -> Self {
        Self { source, cursor: 0 }
    }

    fn next_token(&mut self) -> Option<&'src str> {
        let rest = &self.source[self.cursor..];
        let trimmed = rest.trim_start();
        let skipped = rest.len() - trimmed.len();
        let end = trimmed.find(' ').unwrap_or(trimmed.len());
        if end == 0 {
            return None;
        }
        let token = &trimmed[..end];
        self.cursor += skipped + end;
        Some(token)
    }
}

fn main() {
    let source = String::from("rust ownership advanced");
    let mut parser = Parser::new(&source);

    while let Some(token) = parser.next_token() {
        println!("{token}");
    }
}
