use crate::helper::{assert_around_100ms, HalfMinus1Setup};
use tearup::{tearup, SequentialContextCombinator};

#[test]
fn it_almost_timeout() {
    assert_around_100ms(setup_almost_timeout);
}

type A = SequentialContextCombinator<HalfMinus1Setup, HalfMinus1Setup>;
#[tearup(A)]
fn setup_almost_timeout() {}

mod asyncc {
    use tearup::{tearup, AsyncSequentialContextCombinator, FutureExt};

    use crate::helper::{async_assert_around_100ms, AsyncHalfMinus1Setup};

    #[tokio::test]
    async fn it_almost_timeout() {
        async_assert_around_100ms(move || { async move { setup_almost_timeout().await } }.boxed())
            .await;
    }

    type A = AsyncSequentialContextCombinator<AsyncHalfMinus1Setup, AsyncHalfMinus1Setup>;
    #[tearup(A)]
    async fn setup_almost_timeout() {}
}
