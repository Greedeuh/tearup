use crate::helper::{SlowContext, TooSlowContext};
use tearup::{tearup, SequentialContextCombinator, WaitingContext};

#[test]
#[should_panic]
fn it_barely_timeout_on_first() {
    setup_barely_timeout_on_first();
}

#[test]
#[should_panic]
fn it_barely_timeout_on_second() {
    setup_barely_timeout_on_second();
}

type A = SequentialContextCombinator<TooSlowContext, SlowContext>;
#[tearup(A)]
fn setup_barely_timeout_on_first() {}

type B = SequentialContextCombinator<SlowContext, TooSlowContext>;
#[tearup(B)]
fn setup_barely_timeout_on_second() {}
