use crate::helper::{assert_around_100ms, SlowSetup};
use tearup::{tearup, ConcurrentContextCombinator};

#[test]
fn it_almost_timeout() {
    assert_around_100ms(setup_almost_timeout);
}

type A = ConcurrentContextCombinator<SlowSetup, SlowSetup>;
#[tearup(A)]
fn setup_almost_timeout() {}

mod asyncc {
    use tearup::{tearup, AsyncConcurrentContextCombinator, FutureExt};

    use crate::helper::{async_assert_around_100ms, AsyncSlowSetup};

    #[tokio::test]
    async fn it_almost_timeout() {
        async_assert_around_100ms(move || { async move { setup_almost_timeout().await } }.boxed())
            .await;
    }

    type A = AsyncConcurrentContextCombinator<AsyncSlowSetup, AsyncSlowSetup>;
    #[tearup(A)]
    async fn setup_almost_timeout() {}
}
