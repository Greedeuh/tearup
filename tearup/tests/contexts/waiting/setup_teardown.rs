use lazy_static::lazy_static;
use std::{
    sync::Mutex,
    thread::{sleep, spawn},
    time::{Duration, SystemTime},
};
use stopwatch::Stopwatch;
use tearup::{tearup, ReadyChecksConfig, ReadyFn, WaitingContext};

use crate::helper::assert_around_100ms;

lazy_static! {
    static ref SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    static ref TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
}

#[test]
fn it_pass_through_setup_then_teardown() {
    let stopwatch = Stopwatch::start_new();

    teardown_panic();

    let raw_setup_checkpoint = SETUP_CHECKPOINT.lock().unwrap().unwrap();
    let raw_teardown_checkpoint = TEARDOWN_CHECKPOINT.lock().unwrap().unwrap();

    assert!(raw_setup_checkpoint < raw_teardown_checkpoint);

    assert_around_100ms(&stopwatch);
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

    fn teardown(&mut self) {
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
    use stopwatch::Stopwatch;
    use tearup::{tearup, AsyncWaitingContext, ReadyChecksConfig, ReadyFn};
    use tokio::{spawn, sync::Mutex, time::sleep};

    use crate::helper::assert_around_100ms;

    lazy_static! {
        static ref SETUP_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
        static ref TEARDOWN_CHECKPOINT: Mutex<Option<SystemTime>> = None.into();
    }

    #[tokio::test]
    async fn it_pass_through_setup_then_teardown() {
        let stopwatch = Stopwatch::start_new();

        teardown_panic().await;

        let raw_setup_checkpoint = SETUP_CHECKPOINT.lock().await.unwrap();
        let raw_teardown_checkpoint = TEARDOWN_CHECKPOINT.lock().await.unwrap();

        assert!(raw_setup_checkpoint < raw_teardown_checkpoint);

        assert_around_100ms(&stopwatch);
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

        async fn teardown(&mut self) {
            let mut checkpoint = TEARDOWN_CHECKPOINT.lock().await;
            *checkpoint = Some(SystemTime::now());
        }
    }

    #[tearup(NiceContext)]
    async fn teardown_panic() {}
}
