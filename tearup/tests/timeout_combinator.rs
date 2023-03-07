use tearup::{tearup, Context, ContextCombinator};
mod helper;
use helper::{InstantContext, TooSlowContext};

#[test]
#[should_panic]
fn it_barely_timeout() {
    setup_barely_timeout()
}

#[test]
#[should_panic]
fn it_barely_timeout_reversed() {
    setup_barely_timeout_reversed()
}

type A = ContextCombinator<TooSlowContext, InstantContext>;
#[tearup(A)]
fn setup_barely_timeout() {}

type B = ContextCombinator<InstantContext, TooSlowContext>;
#[tearup(B)]
fn setup_barely_timeout_reversed() {}

#[cfg(feature = "async")]
mod asyncc {}
