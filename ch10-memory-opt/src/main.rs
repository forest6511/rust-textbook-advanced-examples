use ch10_memory_opt::{
    arena, capacity, enum_opt, layout, log_parser, maybe_uninit, repr_demo,
    small_vec_demo,
};

fn main() {
    println!("== layout ==");
    layout::print_primitive_sizes();
    layout::print_alignment();

    println!("\n== repr ==");
    repr_demo::print_repr_sizes();

    println!("\n== enum sizes ==");
    enum_opt::print_enum_sizes();

    println!("\n== capacity ==");
    let items: Vec<u32> = (0..1024).collect();
    let v1 = capacity::collect_with_default(&items);
    let v2 = capacity::collect_with_capacity(&items);
    println!(
        "default cap = {}, with_capacity cap = {}",
        v1.capacity(),
        v2.capacity(),
    );

    let lines: Vec<&str> = (0..3).map(|_| "a,b,c,d,e").collect();
    let n1 = capacity::split_per_iter(&lines);
    let n2 = capacity::split_reuse(&lines);
    println!("split_per_iter = {n1}, split_reuse = {n2}");

    println!("\n== smallvec ==");
    small_vec_demo::show_layout();
    let workloads: Vec<&[i32]> = vec![&[1, 2], &[1, 2, 3, 4], &[1, 2, 3, 4, 5]];
    let (spilled, total) = small_vec_demo::measure_spill_rate(&workloads);
    println!("spilled {spilled}/{total}");
    let inline = small_vec_demo::build_inline_demo();
    println!(
        "inline demo: {:?}, spilled? {}",
        &inline[..],
        inline.spilled(),
    );

    println!("\n== arena ==");
    arena::run_arena_demo();

    println!("\n== maybe_uninit ==");
    let a = maybe_uninit::make_array_from_fn();
    let b = maybe_uninit::make_array_maybe_uninit();
    println!("from_fn       = {a:?}");
    println!("maybe_uninit  = {b:?}");

    println!("\n== log_parser ==");
    let data = log_parser::sample_log();
    let n_naive = log_parser::parse_naive(data.as_bytes());
    let n_reuse = log_parser::parse_reuse(data.as_bytes());
    println!("ERROR lines: naive={n_naive}, reuse={n_reuse}");
}
