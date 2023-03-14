use tearup::{tearup, Context, SequentialContextCombinator};
mod helper;
use helper::{HalfPlus1Context, SlowContext, TooSlowContext};

// #[test]
// #[should_panic]
fn it_barely_timeout() {
    use stopwatch::Stopwatch;
    let sw = Stopwatch::start_new();

    setup_barely_timeout();

    println!("Thing took {}ms", sw.elapsed_ms());
}

type A = SequentialContextCombinator<HalfPlus1Context, HalfPlus1Context>;
#[tearup(A)]
fn setup_barely_timeout() {}
