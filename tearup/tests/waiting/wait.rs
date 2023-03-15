use crate::helper::SlowContext;
use tearup::tearup;

#[test]
fn it_almost_timeout() {
    setup_almost_timeout()
}

#[tearup(SlowContext)]
fn setup_almost_timeout() {}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::tearup;

    use crate::helper::AsyncSlowContext;

    #[tokio::test]
    async fn it_almost_timeout() {
        setup_almost_timeout().await
    }

    #[tearup(AsyncSlowContext)]
    async fn setup_almost_timeout() {}
}