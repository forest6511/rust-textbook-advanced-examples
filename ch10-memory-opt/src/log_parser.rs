use std::io::{BufRead, BufReader, Read};

pub fn parse_naive<R: Read>(reader: R) -> usize {
    let buf = BufReader::new(reader);
    let mut total = 0;
    for line in buf.lines() {
        let line = line.expect("read_line failed");
        if line.contains("ERROR") {
            total += 1;
        }
    }
    total
}

pub fn parse_reuse<R: Read>(reader: R) -> usize {
    let mut buf = BufReader::new(reader);
    let mut line = String::new();
    let mut total = 0;
    loop {
        line.clear();
        let n = buf.read_line(&mut line).expect("read_line failed");
        if n == 0 {
            break;
        }
        if line.contains("ERROR") {
            total += 1;
        }
    }
    total
}

pub fn sample_log() -> String {
    let mut s = String::with_capacity(1024 * 64);
    for i in 0..1000 {
        let level = if i % 7 == 0 { "ERROR" } else { "INFO" };
        s.push_str(&format!("2026-05-04 10:00:{i:02} {level} request\n"));
    }
    s
}
