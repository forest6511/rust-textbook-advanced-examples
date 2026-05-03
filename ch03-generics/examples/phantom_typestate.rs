//! `PhantomData<T>` で型レベルの状態を持たせる typestate パターン。
//!
//! Connection<Closed> と Connection<Open> は実体としては同じデータだが、
//! 型レベルで状態が違うので、メソッドを分離できる。閉じた接続に対して
//! query() を呼ぶようなコードはコンパイルで弾ける。

use std::marker::PhantomData;

pub struct Closed;
pub struct Open;

pub struct Connection<S> {
    endpoint: String,
    _state: PhantomData<S>,
}

impl Connection<Closed> {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            _state: PhantomData,
        }
    }

    pub fn open(self) -> Connection<Open> {
        println!("opening {}", self.endpoint);
        Connection {
            endpoint: self.endpoint,
            _state: PhantomData,
        }
    }
}

impl Connection<Open> {
    pub fn query(&self, sql: &str) -> String {
        format!("[{}] {}", self.endpoint, sql)
    }

    pub fn close(self) -> Connection<Closed> {
        println!("closing {}", self.endpoint);
        Connection {
            endpoint: self.endpoint,
            _state: PhantomData,
        }
    }
}

fn main() {
    let conn = Connection::<Closed>::new("postgres://localhost");
    let conn = conn.open();
    println!("rows: {}", conn.query("SELECT 1"));
    let _ = conn.close();
    // 次の行を有効にするとコンパイルエラー: closed 状態には query が無い。
    // conn.query("SELECT 2");
}
