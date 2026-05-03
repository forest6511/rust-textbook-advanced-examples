// 「ロガー抽象化」を題材に、動的版と静的版の同等な API を並べる。
//
// 動的版 (Arc<dyn Logger>) は実行時に実装を差し替えられる。
// 静的版 (StaticService<L>) はコンパイル時に型が確定し、
// 単相化により呼び出しがインライン化される。
//
// 設計判断は「実行時切替の必要性 vs 性能/単相化のメリット」のトレードオフ。

use std::sync::Arc;

trait Logger: Send + Sync {
    fn log(&self, level: &str, msg: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, level: &str, msg: &str) {
        println!("[{level}] {msg}");
    }
}

struct PrefixLogger {
    name: String,
}

impl Logger for PrefixLogger {
    fn log(&self, level: &str, msg: &str) {
        println!("({}) [{level}] {msg}", self.name);
    }
}

// 動的版: フィールドの型は Arc<dyn Logger>
struct DynamicService {
    logger: Arc<dyn Logger>,
}

impl DynamicService {
    fn new(logger: Arc<dyn Logger>) -> Self {
        Self { logger }
    }

    fn handle(&self, msg: &str) {
        self.logger.log("info", msg);
    }
}

// 静的版: 型パラメータでロガーを受け取り、単相化される
struct StaticService<L: Logger> {
    logger: L,
}

impl<L: Logger> StaticService<L> {
    fn new(logger: L) -> Self {
        Self { logger }
    }

    fn handle(&self, msg: &str) {
        self.logger.log("info", msg);
    }
}

fn main() {
    // 動的版: 実行時の条件で実装を選ぶ
    let logger: Arc<dyn Logger> = if std::env::var("VERBOSE").is_ok() {
        Arc::new(PrefixLogger {
            name: "verbose".into(),
        })
    } else {
        Arc::new(ConsoleLogger)
    };
    let svc = DynamicService::new(logger);
    svc.handle("started (dynamic)");

    // 静的版: 型ごとに別の StaticService<...> がコンパイル時に生成される
    let svc = StaticService::new(ConsoleLogger);
    svc.handle("started (static, Console)");

    let svc = StaticService::new(PrefixLogger {
        name: "static".into(),
    });
    svc.handle("started (static, Prefix)");
}
