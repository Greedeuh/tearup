use tearup::{tearup, Context, SequentialContextCombinator};
mod helper;
use helper::HalfMinus1Context;

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

type A = SequentialContextCombinator<HalfMinus1Context, HalfMinus1Context>;
#[tearup(A)]
fn setup_almost_timeout() {}
