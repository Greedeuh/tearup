use crate::helper::{assert_timeout_around_100ms, TooSlowTeardown};
use tearup::tearup;

#[test]
fn it_barely_timeout() {
    assert_timeout_around_100ms(setup_barely_timeout);
}

#[tearup(TooSlowTeardown)]
fn setup_barely_timeout() {}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::{tearup, FutureExt};

    use crate::helper::{async_assert_timeout_around_100ms, AsyncTooSlowTeardown};

    #[tokio::test]
    async fn it_barely_timeout() {
        async_assert_timeout_around_100ms(move || {
            { async move { setup_barely_timeout().await } }.boxed()
        })
        .await;
    }

    #[tearup(AsyncTooSlowTeardown)]
    async fn setup_barely_timeout() {}
}