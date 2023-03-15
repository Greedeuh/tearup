use crate::helper::HalfMinus1Context;
use tearup::{tearup, ContextCombinator};

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

type A = ContextCombinator<HalfMinus1Context, HalfMinus1Context>;
#[tearup(A)]
fn setup_almost_timeout() {}
