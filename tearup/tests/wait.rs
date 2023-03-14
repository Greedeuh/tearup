use tearup::{tearup, WaitingContext};
mod helper;
use helper::SlowContext;

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

#[tearup(SlowContext)]
fn setup_almost_timeout() {}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::{tearup, AsyncContext};

    use crate::helper::AsyncSlowContext;

    #[tokio::test]
    async fn it_almost_timeout() {
        setup_almost_timeout().await
    }

    #[tearup(AsyncSlowContext)]
    async fn setup_almost_timeout() {}
}
