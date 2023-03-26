use lazy_static::lazy_static;
use std::{
    thread::{sleep, spawn},
    time::{Duration, SystemTime},
};
use tearup::{tearup, ReadyChecksConfig, ReadyFn, WaitingContext};

use crate::helper::{assert_around_100ms, assert_order, Checkpoint};

lazy_static! {
    static ref SETUP_CHECKPOINT: Checkpoint = None.into();
    static ref TEARDOWN_CHECKPOINT: Checkpoint = None.into();
}

#[test]
fn it_pass_through_setup_then_teardown() {
    assert_around_100ms(setup_then_teardown);
    assert_order(&SETUP_CHECKPOINT, &TEARDOWN_CHECKPOINT);
}

struct NiceContext;
impl WaitingContext for NiceContext {
    fn ready_checks_config(&self) -> ReadyChecksConfig {
        ReadyChecksConfig {
            duration: Duration::from_millis(5),
            maximum: 100,
        }
    }

    fn setup(ready: ReadyFn) -> Self {
        let mut checkpoint = SETUP_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
        spawn(move || {
            sleep(Duration::from_millis(100));
            ready();
        });

        Self {}
    }

    fn teardown(self, ready: ReadyFn) {
        let mut checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap();
        *checkpoint = Some(SystemTime::now());
        ready();
    }
}

#[tearup(NiceContext)]
fn setup_then_teardown() {}

#[cfg(feature = "async")]
mod asyncc {
    use async_trait::async_trait;
    use lazy_static::lazy_static;
    use std::time::{Duration, SystemTime};
    use tearup::{tearup, AsyncWaitingContext, FutureExt, ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, time::sleep};

    use crate::helper::{assert_async_order, async_assert_around_100ms, AsyncCheckpoint};

    lazy_static! {
        static ref SETUP_CHECKPOINT: AsyncCheckpoint = None.into();
        static ref TEARDOWN_CHECKPOINT: AsyncCheckpoint = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown() {
        async_assert_around_100ms(move || { async move { setup_then_teardown().await } }.boxed())
            .await;

        assert_async_order(&SETUP_CHECKPOINT, &TEARDOWN_CHECKPOINT).await;
    }

    struct NiceContext;
    #[async_trait]
    impl AsyncWaitingContext<'_> for NiceContext {
        fn ready_checks_config(&self) -> ReadyChecksConfig {
            ReadyChecksConfig::ms500()
        }

        async fn setup(ready: ReadyFn) -> Self {
            let mut checkpoint = SETUP_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
            spawn(async move {
                sleep(Duration::from_millis(100)).await;
                ready();
            })
            .await
            .unwrap();
            Self {}
        }

        async fn teardown(mut self, ready: ReadyFn) {
            let mut checkpoint = TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
            ready();
        }
    }

    #[tearup(NiceContext)]
    async fn setup_then_teardown() {}
}
