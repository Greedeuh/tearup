use lazy_static::lazy_static;
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};
use tearup::{tearup, SharedContext, SimpleContext};

use crate::helper::{assert_order, Checkpoint};

lazy_static! {
    static ref SETUP_CHECKPOINT: Checkpoint = None.into();
    static ref TEARDOWN_CHECKPOINT: Checkpoint = None.into();
}

#[test]
fn it_pass_through_setup_then_teardown() {
    teardown_panic();

    assert_order(&SETUP_CHECKPOINT, &TEARDOWN_CHECKPOINT);
}

struct NiceContext;
impl SimpleContext for NiceContext {
    fn setup(_shared_context: &mut SharedContext) -> Self {
        let mut checkpoint = SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());

        sleep(Duration::from_millis(10));

        Self {}
    }

    fn teardown(self) {
        let mut checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
    }
}

#[tearup(NiceContext)]
fn teardown_panic() {}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use lazy_static::lazy_static;
    use std::time::{Duration, SystemTime};
    use tearup::{tearup, AsyncSimpleContext};
    use tokio::time::sleep;

    use crate::helper::{assert_async_order, AsyncCheckpoint};

    lazy_static! {
        static ref SETUP_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref TEARDOWN_CHECKPOINT: AsyncCheckpoint = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown() {
        teardown_panic().await;

        assert_async_order(&SETUP_CHECKPOINT, &TEARDOWN_CHECKPOINT).await;
    }

    struct NiceContext;
    #[async_trait]
    impl AsyncSimpleContext<'_> for NiceContext {
        async fn setup() -> Self {
            let mut checkpoint = SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());

            sleep(Duration::from_millis(10)).await;

            Self {}
        }

        async fn teardown(mut self) {
            let mut checkpoint = TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }

    #[tearup(NiceContext)]
    async fn teardown_panic() {}
}
