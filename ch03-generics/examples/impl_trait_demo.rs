//! `impl Trait` を引数位置と戻り値位置で使い分ける。
//!
//! - 引数位置: 匿名のジェネリックパラメータ。呼び出し側が型を決める。
//! - 戻り値位置 (RPIT): 関数側が「ある具象型」を返す。呼び出し側からは
//!   トレイトメソッドしか見えず、具象型は隠蔽される。
//!
//! 戻り値位置で重要なのは「分岐ごとに違う具象型は返せない」こと。
//! Vec を返すか Iterator を返すかで型が異なるなら Box<dyn Iterator> が必要。

fn sum_squares(nums: impl Iterator<Item = u32>) -> u32 {
    nums.map(|n| n * n).sum()
}

fn even_numbers(upto: u32) -> impl Iterator<Item = u32> {
    (0..upto).filter(|n| n % 2 == 0)
}

// 戻り値の RPIT は「単一の具象型」を返す必要がある。下の関数は条件分岐で
// 異なる Iterator 具象型 (Filter / Map) を返したくなるが、コンパイルが通らない。
// その場合は Box<dyn Iterator<...>> を使う。
fn pick_iter(filter: bool, upto: u32) -> Box<dyn Iterator<Item = u32>> {
    if filter {
        Box::new((0..upto).filter(|n| n % 2 == 0))
    } else {
        Box::new((0..upto).map(|n| n + 1))
    }
}

fn main() {
    let total = sum_squares(0..5);
    println!("sum  : {total}");

    let head: Vec<u32> = even_numbers(10).take(3).collect();
    println!("even : {head:?}");

    let dyn_iter = pick_iter(true, 6);
    let collected: Vec<u32> = dyn_iter.collect();
    println!("pick : {collected:?}");
}
