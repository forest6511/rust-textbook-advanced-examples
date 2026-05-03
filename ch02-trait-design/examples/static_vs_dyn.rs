// 静的ディスパッチ (impl Trait) と動的ディスパッチ (dyn Trait) を 1 つの
// Logger trait で対比する例。
//
// 静的版はコンパイル時に型が確定するためインライン化が効く。
// 動的版は vtable 経由で呼び出すため、Vec<Box<dyn Logger>> のように
// 異種の実装を 1 つのコレクションに詰めて回せる。

trait Logger {
    fn log(&self, message: &str);
}

struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[console] {message}");
    }
}

struct PrefixLogger {
    prefix: &'static str,
}

impl Logger for PrefixLogger {
    fn log(&self, message: &str) {
        println!("[{}] {message}", self.prefix);
    }
}

// 静的ディスパッチ: 単相化されて型ごとに別実装が生成される。
fn run_static<L: Logger>(logger: &L, message: &str) {
    logger.log(message);
}

// 動的ディスパッチ: 関数本体は 1 つだけ。vtable 経由で呼び出す。
fn run_dynamic(logger: &dyn Logger, message: &str) {
    logger.log(message);
}

fn main() {
    let console = ConsoleLogger;
    let prefix = PrefixLogger { prefix: "info" };

    run_static(&console, "static + ConsoleLogger");
    run_static(&prefix, "static + PrefixLogger");

    run_dynamic(&console, "dynamic + ConsoleLogger");
    run_dynamic(&prefix, "dynamic + PrefixLogger");

    let loggers: Vec<Box<dyn Logger>> = vec![
        Box::new(ConsoleLogger),
        Box::new(PrefixLogger { prefix: "warn" }),
    ];
    for logger in &loggers {
        logger.log("from heterogeneous Vec");
    }
}
