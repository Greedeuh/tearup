use crate::helper::{assert_timeout_around_100ms, InstantContext, TooSlowContext};
use tearup::{tearup, SequentialContextCombinator};

#[test]
fn it_barely_timeout_on_first() {
    assert_timeout_around_100ms(setup_barely_timeout_on_first);
}

#[test]
fn it_barely_timeout_on_second() {
    assert_timeout_around_100ms(setup_barely_timeout_on_second);
}

type A = SequentialContextCombinator<TooSlowContext, InstantContext>;
#[tearup(A)]
fn setup_barely_timeout_on_first() {}

type B = SequentialContextCombinator<InstantContext, TooSlowContext>;
#[tearup(B)]
fn setup_barely_timeout_on_second() {}

mod asyncc {
    use crate::helper::{
        async_assert_timeout_around_100ms, AsyncInstantContext, AsyncTooSlowContext,
    };
    use tearup::{tearup, AsyncSequentialContextCombinator, FutureExt};

    #[tokio::test]
    async fn it_barely_timeout_on_first() {
        async_assert_timeout_around_100ms(move || {
            { async move { setup_barely_timeout_on_first().await } }.boxed()
        })
        .await;
    }

    #[tokio::test]
    async fn it_barely_timeout_on_second() {
        async_assert_timeout_around_100ms(move || {
            { async move { setup_barely_timeout_on_second().await } }.boxed()
        })
        .await;
    }

    type A = AsyncSequentialContextCombinator<AsyncTooSlowContext, AsyncInstantContext>;
    #[tearup(A)]
    async fn setup_barely_timeout_on_first() {}

    type B = AsyncSequentialContextCombinator<AsyncInstantContext, AsyncTooSlowContext>;
    #[tearup(B)]
    async fn setup_barely_timeout_on_second() {}
}
