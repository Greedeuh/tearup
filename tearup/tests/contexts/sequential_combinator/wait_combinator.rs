use crate::helper::{assert_around_100ms, HalfMinus1Context};
use tearup::{tearup, SequentialContextCombinator};

#[test]
fn it_almost_timeout() {
    assert_around_100ms(setup_almost_timeout);
}

type A = SequentialContextCombinator<HalfMinus1Context, HalfMinus1Context>;
#[tearup(A)]
fn setup_almost_timeout() {}

mod asyncc {
    use tearup::{tearup, AsyncSequentialContextCombinator, FutureExt};

    use crate::helper::{async_assert_around_100ms, AsyncHalfMinus1Context};

    #[tokio::test]
    async fn it_almost_timeout() {
        async_assert_around_100ms(move || { async move { setup_almost_timeout().await } }.boxed())
            .await;
    }

    type A = AsyncSequentialContextCombinator<AsyncHalfMinus1Context, AsyncHalfMinus1Context>;
    #[tearup(A)]
    async fn setup_almost_timeout() {}
}
