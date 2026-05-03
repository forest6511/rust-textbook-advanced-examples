//! typestate Builder: 必須フィールドが揃わないと build() を呼べない。
//!
//! ServerBuilder<HasHost, HasPort> の HasHost / HasPort はマーカー型で、
//! 「設定済みかどうか」を型レベルで持つ。build() は両方が Set のときだけ
//! 実装される。実行時チェックではなくコンパイル時に必須項目漏れを検出する。

use std::marker::PhantomData;

pub struct Set;
pub struct Unset;

pub struct ServerBuilder<H, P> {
    host: Option<String>,
    port: Option<u16>,
    _marker: PhantomData<(H, P)>,
}

impl ServerBuilder<Unset, Unset> {
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
            _marker: PhantomData,
        }
    }
}

impl Default for ServerBuilder<Unset, Unset> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P> ServerBuilder<Unset, P> {
    pub fn host(self, host: impl Into<String>) -> ServerBuilder<Set, P> {
        ServerBuilder {
            host: Some(host.into()),
            port: self.port,
            _marker: PhantomData,
        }
    }
}

impl<H> ServerBuilder<H, Unset> {
    pub fn port(self, port: u16) -> ServerBuilder<H, Set> {
        ServerBuilder {
            host: self.host,
            port: Some(port),
            _marker: PhantomData,
        }
    }
}

impl ServerBuilder<Set, Set> {
    pub fn build(self) -> Server {
        Server {
            host: self.host.expect("typestate guarantees host is set"),
            port: self.port.expect("typestate guarantees port is set"),
        }
    }
}

pub struct Server {
    pub host: String,
    pub port: u16,
}

fn main() {
    let server = ServerBuilder::new()
        .host("127.0.0.1")
        .port(8080)
        .build();
    println!("server: {}:{}", server.host, server.port);

    // 次のコードはコンパイルエラー: port が Unset なので build() が見つからない。
    // let bad = ServerBuilder::new().host("127.0.0.1").build();
}
