use crate::helper::{SlowContext, TooSlowContext};
use tearup::{tearup, SequentialContextCombinator};

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

mod asyncc {
    use crate::helper::{AsyncSlowContext, AsyncTooSlowContext};
    use tearup::{tearup, AsyncSequentialContextCombinator};

    #[tokio::test]
    #[should_panic]
    async fn it_barely_timeout_on_first() {
        setup_barely_timeout_on_first().await;
    }

    #[tokio::test]
    #[should_panic]
    async fn it_barely_timeout_on_second() {
        setup_barely_timeout_on_second().await;
    }

    type A = AsyncSequentialContextCombinator<AsyncTooSlowContext, AsyncSlowContext>;
    #[tearup(A)]
    async fn setup_barely_timeout_on_first() {}

    type B = AsyncSequentialContextCombinator<AsyncSlowContext, AsyncTooSlowContext>;
    #[tearup(B)]
    async fn setup_barely_timeout_on_second() {}
}
