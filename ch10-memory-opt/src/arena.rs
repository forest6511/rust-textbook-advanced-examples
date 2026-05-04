use bumpalo::Bump;
use bumpalo::collections::Vec as BumpVec;

pub fn parse_into_arena<'bump>(
    arena: &'bump Bump,
    input: &str,
) -> BumpVec<'bump, &'bump str> {
    let mut tokens = BumpVec::new_in(arena);
    for word in input.split_whitespace() {
        let owned: &str = arena.alloc_str(word);
        tokens.push(owned);
    }
    tokens
}

pub fn run_arena_demo() {
    let mut arena = Bump::new();
    for round in 0..3 {
        let input = "the quick brown fox jumps";
        let tokens = parse_into_arena(&arena, input);
        println!("round {}: tokens = {:?}", round, &tokens[..]);
        drop(tokens);
        arena.reset();
    }
    println!("arena allocated bytes (final): {}", arena.allocated_bytes());
}
