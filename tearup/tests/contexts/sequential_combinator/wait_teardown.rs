use crate::helper::{assert_around_100ms, HalfMinus1Teardown};
use tearup::{tearup, SequentialContextCombinator};

#[test]
fn it_almost_timeout() {
    assert_around_100ms(teardown_almost_timeout);
}

type A = SequentialContextCombinator<HalfMinus1Teardown, HalfMinus1Teardown>;
#[tearup(A)]
fn teardown_almost_timeout() {}

mod asyncc {
    use tearup::{tearup, AsyncSequentialContextCombinator, FutureExt};

    use crate::helper::{async_assert_around_100ms, AsyncHalfMinus1Setup};

    #[tokio::test]
    async fn it_almost_timeout() {
        async_assert_around_100ms(move || {
            { async move { teardown_almost_timeout().await } }.boxed()
        })
        .await;
    }

    type A = AsyncSequentialContextCombinator<AsyncHalfMinus1Setup, AsyncHalfMinus1Setup>;
    #[tearup(A)]
    async fn teardown_almost_timeout() {}
}
