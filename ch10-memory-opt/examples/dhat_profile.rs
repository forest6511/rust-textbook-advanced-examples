use ch10_memory_opt::log_parser;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "naive".to_string());
    let _profiler = dhat::Profiler::new_heap();
    let data = log_parser::sample_log();
    let count = match mode.as_str() {
        "reuse" => log_parser::parse_reuse(data.as_bytes()),
        _ => log_parser::parse_naive(data.as_bytes()),
    };
    eprintln!("mode={mode}, ERROR lines={count}");
}
