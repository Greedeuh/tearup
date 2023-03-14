use tearup::{tearup, ContextCombinator, WaitingContext};
mod helper;
use helper::SlowContext;

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

type A = ContextCombinator<SlowContext, SlowContext>;
#[tearup(A)]
fn setup_almost_timeout() {}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::{tearup, AsyncContext, AsyncContextCombinator};

    use crate::helper::AsyncSlowContext;

    #[tokio::test]
    async fn it_almost_timeout() {
        setup_almost_timeout().await
    }

    type A = AsyncContextCombinator<AsyncSlowContext, AsyncSlowContext>;
    #[tearup(A)]
    async fn setup_almost_timeout() {}
}
