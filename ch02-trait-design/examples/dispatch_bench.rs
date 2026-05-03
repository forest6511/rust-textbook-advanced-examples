// 静的ディスパッチと動的ディスパッチの簡易性能比較。
//
// 同じ Op trait を impl Trait（静的）と &dyn Trait（動的）で 1000万回呼び、
// 経過時間を Instant で計測する。CPU bound では vtable 経由の間接呼び出しコストが
// 加算演算 1 回より高くつく場面が多いため、静的版が有利になりやすい。
//
// 厳密なベンチは Ch.9 の criterion で扱う。本例は「桁感を掴む」目的。

use std::hint::black_box;
use std::time::Instant;

trait Op {
    fn apply(&self, x: u64) -> u64;
}

struct AddOne;

impl Op for AddOne {
    fn apply(&self, x: u64) -> u64 {
        x.wrapping_add(1)
    }
}

const N: u64 = 10_000_000;

fn run_static<O: Op>(op: &O) -> u64 {
    let mut acc: u64 = 0;
    for i in 0..N {
        acc = acc.wrapping_add(black_box(op.apply(i)));
    }
    acc
}

fn run_dynamic(op: &dyn Op) -> u64 {
    let mut acc: u64 = 0;
    for i in 0..N {
        acc = acc.wrapping_add(black_box(op.apply(i)));
    }
    acc
}

fn main() {
    let op = AddOne;

    let t = Instant::now();
    let s = run_static(&op);
    let static_elapsed = t.elapsed();

    let t = Instant::now();
    let d = run_dynamic(&op);
    let dynamic_elapsed = t.elapsed();

    println!("static  : {static_elapsed:?} (sum={s})");
    println!("dynamic : {dynamic_elapsed:?} (sum={d})");
}
