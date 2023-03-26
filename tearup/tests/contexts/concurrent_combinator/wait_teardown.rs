use crate::helper::{assert_around_100ms, SlowTeardown};
use tearup::{tearup, ConcurrentContextCombinator};

#[test]
fn it_almost_timeout() {
    assert_around_100ms(setup_almost_timeout);
}

type A = ConcurrentContextCombinator<SlowTeardown, SlowTeardown>;
#[tearup(A)]
fn setup_almost_timeout() {}

mod asyncc {
    use tearup::{tearup, AsyncConcurrentContextCombinator, FutureExt};

    use crate::helper::{async_assert_around_100ms, AsyncSlowTeardown};

    #[tokio::test]
    async fn it_almost_timeout() {
        async_assert_around_100ms(move || { async move { setup_almost_timeout().await } }.boxed())
            .await;
    }

    type A = AsyncConcurrentContextCombinator<AsyncSlowTeardown, AsyncSlowTeardown>;
    #[tearup(A)]
    async fn setup_almost_timeout() {}
}
