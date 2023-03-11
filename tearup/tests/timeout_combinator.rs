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
mod asyncc {
    use tearup::{tearup, AsyncContext, AsyncContextCombinator};

    use crate::helper::{AsyncInstantContext, AsyncTooSlowContext};

    #[tokio::test]
    #[should_panic]
    async fn it_barely_timeout() {
        setup_barely_timeout().await
    }

    #[tokio::test]
    #[should_panic]
    async fn it_barely_timeout_reversed() {
        setup_barely_timeout_reversed().await
    }

    type A = AsyncContextCombinator<AsyncTooSlowContext, AsyncInstantContext>;
    #[tearup(A)]
    async fn setup_barely_timeout() {}

    type B = AsyncContextCombinator<AsyncInstantContext, AsyncTooSlowContext>;
    #[tearup(B)]
    async fn setup_barely_timeout_reversed() {}
}
