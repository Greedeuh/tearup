use crate::helper::TooSlowContext;
use tearup::tearup;

#[test]
#[should_panic]
fn it_barely_timeout() {
    setup_barely_timeout()
}

#[tearup(TooSlowContext)]
fn setup_barely_timeout() {}

#[cfg(feature = "async")]
mod asyncc {
    use tearup::{tearup, AsyncContext};

    use crate::helper::AsyncTooSlowContext;

    #[tokio::test]
    #[should_panic]
    async fn it_barely_timeout() {
        setup_barely_timeout().await
    }

    #[tearup(AsyncTooSlowContext)]
    async fn setup_barely_timeout() {}
}
