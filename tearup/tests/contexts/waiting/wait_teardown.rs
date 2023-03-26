use crate::helper::{assert_around_100ms, SlowTeardown};
use tearup::tearup;

#[test]
fn it_almost_timeout() {
    assert_around_100ms(teardown_almost_timeout);
}

#[tearup(SlowTeardown)]
fn teardown_almost_timeout() {}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::{tearup, FutureExt};

    use crate::helper::{async_assert_around_100ms, AsyncSlowTeardown};

    #[tokio::test]
    async fn it_almost_timeout() {
        async_assert_around_100ms(move || {
            { async move { teardown_almost_timeout().await } }.boxed()
        })
        .await;
    }

    #[tearup(AsyncSlowTeardown)]
    async fn teardown_almost_timeout() {}
}
