use crate::helper::HalfMinus1Context;
use tearup::{tearup, SequentialContextCombinator, WaitingContext};

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

type A = SequentialContextCombinator<HalfMinus1Context, HalfMinus1Context>;
#[tearup(A)]
fn setup_almost_timeout() {}
