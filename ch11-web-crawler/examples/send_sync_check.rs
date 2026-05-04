//! Send / Sync 実装チェック
//!
//! 本書 Ch.11 の主張を `cargo build --example send_sync_check` で実証する。
//!
//! 各型の auto trait 実装は次のとおり:
//!
//! | 型                       | Send  | Sync  |
//! |--------------------------|-------|-------|
//! | `scraper::Html`          | !Send | !Sync |
//! | `scraper::Selector`      | Send  | Sync  |
//! | `rusqlite::Connection`   | Send  | !Sync |
//!
//! - `Html` の !Send は `tendril::NonAtomic`（`Cell<usize>`）由来。
//!   `Tendril<UTF8>: Send` の境界が `NonAtomic: Sync` を要求し、
//!   `Cell<usize>: !Sync` で連鎖的に脱落する。
//! - `Connection` の !Sync は内部の `RefCell<InnerConnection>`,
//!   `RefCell<LruCache<...>>` 由来。Send は `unsafe impl` で明示。
//!
//! 反例（コンパイルエラーで確認）はコメントアウトしてある。
//! 確認したい場合はそれぞれの assert を有効にすると、cargo build がエラーで
//! 「`Cell<usize> cannot be shared between threads safely`」「`RefCell<...>
//! cannot be shared between threads safely`」と教えてくれる。

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    // PASS する主張
    assert_send::<scraper::Selector>();
    assert_sync::<scraper::Selector>();
    assert_send::<rusqlite::Connection>();

    // 反例（有効化するとコンパイルエラー）:
    // assert_send::<scraper::Html>();          // Html: !Send
    // assert_sync::<scraper::Html>();          // Html: !Sync
    // assert_sync::<rusqlite::Connection>();   // Connection: !Sync
}
